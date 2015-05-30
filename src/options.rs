use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Options {
    pub config: Option<PathBuf>,
}

impl Options {
    #[inline]
    pub fn new() -> Options {
        Default::default()
    }

    pub fn set(&mut self, name: &str, value: String) -> bool {
        match name {
            "config" => self.config = Some(PathBuf::from(value)),
            _ => return false,
        }
        true
    }
}
