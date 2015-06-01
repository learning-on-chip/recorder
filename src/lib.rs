extern crate mcpat;
extern crate sqlite;

use std::fmt::Display;

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

mod experiment;
mod options;

use experiment::Experiment;
pub use options::Options;

#[inline]
pub fn process(options: Options) -> Result<()> {
    try!(Experiment::new(options)).run()
}
