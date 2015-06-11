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

mod options;
mod system;

pub mod database;
pub mod server;

pub use options::{Options, OptionValue};
pub use system::System;
