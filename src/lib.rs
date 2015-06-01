extern crate mcpat;
extern crate sqlite;
extern crate threed_ice as ice;

use std::fs;
use std::path::Path;

mod options;

pub use options::Options;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = &'static str;

macro_rules! raise(
    ($message:expr) => (
        return Err($message);
    );
);

pub fn process(options: Options) -> Result<()> {
    match options.config {
        Some(ref config) => if !exists(config) {
            raise!("the configuration file does not exist");
        },
        None => raise!("a configuration file is required"),
    }
    Ok(())
}

#[inline]
fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}
