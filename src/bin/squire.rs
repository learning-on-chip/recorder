#![cfg_attr(test, allow(dead_code, unused_imports))]

extern crate arguments;
extern crate mcpat;

#[macro_use]
extern crate squire;

mod dynamic;
mod statik;
mod support;

use arguments::Arguments;
use std::{env, process};
use std::fmt::Display;
use std::io::{self, Write};

const USAGE: &'static str = "
Usage: squire <command> [options]

Commands:
    dynamic        Record the power consumed of a system on a chip.
    static         Record the area occupied by a system on a chip.

Options:
    --help         Display the usage information of a particular command.
";

fn main() {
    let Arguments { options, orphans, .. } = match arguments::parse(env::args()) {
        Ok(arguments) => arguments,
        _ => usage(USAGE),
    };
    if orphans.len() != 1 {
        usage(USAGE);
    }
    let result = match &orphans[0][..] {
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