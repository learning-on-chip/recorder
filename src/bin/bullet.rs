#![cfg_attr(test, allow(dead_code, unused_imports))]

#[macro_use] extern crate bullet;
extern crate mcpat;
extern crate options;

mod dynamic;
mod statik;
mod support;

use std::{env, process};
use std::fmt::Display;
use std::io::{self, Write};

use bullet::arguments;

const USAGE: &'static str = "
Usage: bullet <command> [options]

Commands:
    dynamic        Record the power consumed of a system on a chip.
    static         Record the area occupied by a system on a chip.

Options:
    --help         Display the usage information of a particular command.
";

fn main() {
    let mut arguments = env::args().skip(1);
    let command = match arguments.next() {
        Some(command) => command,
        _ => usage(USAGE),
    };
    let options = match arguments::parse(arguments) {
        Ok(options) => options,
        Err(error) => fail(error),
    };
    let result = match &command[..] {
        "dynamic" => dynamic::execute(&options),
        "static" => statik::execute(&options),
        _ => fail("the command is unknown"),
    };
    match result {
        Err(error) => fail(error),
        _ => {},
    }
}

#[allow(unused_must_use)]
fn fail<T: Display>(error: T) -> ! {
    io::stderr().write_fmt(format_args!("Error: {}.\n", error));
    process::exit(1);
}

#[allow(unused_must_use)]
fn usage(message: &str) -> ! {
    io::stderr().write_fmt(format_args!("{}\n", message.trim()));
    process::exit(0);
}
