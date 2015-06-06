extern crate hiredis;
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

mod database;
mod options;
mod server;
mod system;
mod worker;

pub use database::Database;
pub use options::Options;
pub use server::Server;
pub use system::System;
pub use worker::Worker;

#[inline]
pub fn process(options: Options) -> Result<()> {
    try!(Worker::new(options)).run()
}
