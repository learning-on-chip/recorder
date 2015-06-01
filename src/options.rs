use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Options {
    pub config: Option<PathBuf>,
    pub database: Option<PathBuf>,
    pub setup: Option<bool>,
}

impl Options {
    #[inline]
    pub fn new() -> Options {
        Default::default()
    }

    pub fn set_flag(&mut self, name: String, value: bool) -> bool {
        match &name[..] {
            "s" | "setup" => self.setup = Some(value),
            _ => return false,
        }
        true
    }

    pub fn set_value(&mut self, name: String, value: String) -> bool {
        match &name[..] {
            "c" | "config" => self.config = Some(PathBuf::from(value)),
            "d" | "database" => self.database = Some(PathBuf::from(value)),
            _ => return false,
        }
        true
    }
}
