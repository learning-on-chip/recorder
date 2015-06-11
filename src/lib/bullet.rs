extern crate hiredis;
extern crate mcpat;
extern crate sqlite;

pub type Address = (String, usize);
pub type Error = Box<std::fmt::Display>;
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! raise(
    ($message:expr) => (
        return Err(Box::new($message));
    );
);

#[macro_export]
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

pub use database::{Database, Recorder};
pub use options::Options;
pub use server::Server;
pub use system::System;
