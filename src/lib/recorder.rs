//! Recording of workload patterns.

extern crate arguments;
extern crate hiredis;
extern crate mcpat;
extern crate sql;
extern crate sqlite;

#[macro_use]
extern crate log;

/// Raise an error.
#[macro_export]
macro_rules! raise(
    ($message:expr) => (return Err($crate::Error::new($message)));
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

mod result;
mod system;

pub mod database;
pub mod server;

pub use result::{Error, Result};
pub use system::System;
