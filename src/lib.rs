extern crate mcpat;
extern crate sqlite;

use std::fmt::Display;
use std::fs;
use std::path::Path;

mod options;

pub use options::Options;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<Display>;

macro_rules! raise(
    ($message:expr) => (
        return Err(Box::new($message));
    );
);

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => raise!(error),
        }
    );
);

pub fn process(options: Options) -> Result<()> {
    let Options { config } = options;

    let config = match config {
        Some(config) => config,
        None => raise!("a configuration file is required"),
    };
    if !exists(&config) {
        raise!("the configuration file does not exist");
    }

    let _system = ok!(mcpat::open(&config));

    Ok(())
}

#[inline]
fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}
