use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Options {
    pub config: Option<PathBuf>,
    pub database: Option<PathBuf>,
    pub caching: Option<(String, usize)>,
    pub prepare: Option<bool>,
}

impl Options {
    #[inline]
    pub fn new() -> Options {
        Default::default()
    }

    pub fn set_flag(&mut self, name: String, value: bool) -> bool {
        match &name[..] {
            "r" | "caching" => self.caching = Some(("localhost".to_string(), 6379)),
            "p" | "prepare" => self.prepare = Some(value),
            _ => return false,
        }
        true
    }

    pub fn set_value(&mut self, name: String, value: String) -> bool {
        match &name[..] {
            "c" | "config" => self.config = Some(PathBuf::from(value)),
            "d" | "database" => self.database = Some(PathBuf::from(value)),
            "r" | "caching" => {
                let chunks = value.split(':').collect::<Vec<_>>();
                let (host, port) = match chunks.len() {
                    1 => match chunks[0].parse::<usize>() {
                        Ok(port) => ("127.0.0.1".to_string(), port),
                        Err(_) => (chunks[0].to_string(), 6379),
                    },
                    2 => match chunks[1].parse::<usize>() {
                        Ok(port) => (chunks[0].to_string(), port),
                        Err(_) => return false,
                    },
                    _ => return false,
                };
                self.caching = Some((host, port));
            },
            _ => return false,
        }
        true
    }
}
