use std::default::Default;
use std::path::PathBuf;

use server;

#[derive(Debug, Default)]
pub struct Options {
    pub server: Option<(String, usize)>,
    pub database: Option<PathBuf>,
    pub caching: Option<bool>,
}

impl Options {
    #[inline]
    pub fn new() -> Options {
        Default::default()
    }

    pub fn set_flag(&mut self, name: String, value: bool) -> bool {
        match &name[..] {
            "c" | "caching" => self.caching = Some(value),
            _ => return false,
        }
        true
    }

    pub fn set_value(&mut self, name: String, value: String) -> bool {
        match &name[..] {
            "s" | "server" => {
                let chunks = value.split(':').collect::<Vec<_>>();
                let (host, port) = match chunks.len() {
                    1 => match chunks[0].parse::<usize>() {
                        Ok(port) => (server::DEFAULT_HOST.to_string(), port),
                        Err(_) => (chunks[0].to_string(), server::DEFAULT_PORT),
                    },
                    2 => match chunks[1].parse::<usize>() {
                        Ok(port) => (chunks[0].to_string(), port),
                        Err(_) => return false,
                    },
                    _ => return false,
                };
                self.server = Some((host, port));
            },
            "d" | "database" => self.database = Some(PathBuf::from(value)),
            _ => return false,
        }
        true
    }
}
