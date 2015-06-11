use std::collections::HashMap;
use std::convert::From;
use std::path::PathBuf;

use {Address, Result};

pub struct Options {
    map: HashMap<String, OptionValue>,
}

pub enum OptionValue {
    Boolean(bool),
    String(String),
}

impl Options {
    pub fn parse<I>(stream: I) -> Result<Options> where I: Iterator<Item=String> {
        macro_rules! truth(
            ($result:expr) => (if !$result {
                raise!("the arguments are invalid");
            });
        );

        let mut map = HashMap::new();
        let mut previous: Option<String> = None;

        for chunk in stream {
            if chunk.starts_with("--") {
                if let Some(name) = previous {
                    map.insert(name, OptionValue::Boolean(true));
                }
                truth!(chunk.len() > 2);
                previous = Some(String::from(&chunk[2..]));
            } else if let Some(name) = previous {
                map.insert(name, OptionValue::String(String::from(chunk)));
                previous = None;
            } else {
                truth!(false);
            }
        }
        if let Some(name) = previous {
            map.insert(name, OptionValue::Boolean(true));
        }

        Ok(Options { map: map })
    }

    #[inline]
    pub fn get<'l, T>(&'l self, name: &str) -> Option<T> where Option<T>: From<&'l OptionValue> {
        self.map.get(name).and_then(|value| value.into())
    }
}

impl<'l> From<&'l OptionValue> for Option<Address> {
    fn from(value: &'l OptionValue) -> Option<Address> {
        match value {
            &OptionValue::String(ref value) => {
                let chunks = value.split(':').collect::<Vec<_>>();
                match chunks.len() {
                    2 => match chunks[1].parse::<usize>() {
                        Ok(port) => Some((chunks[0].to_string(), port)),
                        _ => None,
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }
}

impl<'l> From<&'l OptionValue> for Option<bool> {
    fn from(value: &'l OptionValue) -> Option<bool> {
        match value {
            &OptionValue::Boolean(value) => Some(value),
            _ => None,
        }
    }
}

impl<'l> From<&'l OptionValue> for Option<PathBuf> {
    fn from(value: &'l OptionValue) -> Option<PathBuf> {
        match value {
            &OptionValue::String(ref value) => Some(PathBuf::from(value)),
            _ => None,
        }
    }
}

impl<'l> From<&'l OptionValue> for Option<String> {
    fn from(value: &'l OptionValue) -> Option<String> {
        match value {
            &OptionValue::String(ref value) => Some(value.to_string()),
            _ => None,
        }
    }
}
