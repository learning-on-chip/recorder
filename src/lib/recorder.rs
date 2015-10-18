#[macro_use]
extern crate log;

extern crate arguments;
extern crate hiredis;
extern crate mcpat;
extern crate sql;
extern crate sqlite;

/// An error.
pub type Error = Box<std::fmt::Display>;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

/// Raise an error.
#[macro_export]
macro_rules! raise(
    ($message:expr) => (return Err(Box::new($message)));
);

/// Unwrap a result or raise an error.
#[macro_export]
macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => raise!(error),
        }
    );
);

mod address;
mod system;

pub mod database;
pub mod server;

pub use address::Address;
pub use system::System;
