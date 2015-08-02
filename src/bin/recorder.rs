#![cfg_attr(test, allow(dead_code, unused_imports))]

extern crate arguments;
extern crate log;
extern crate mcpat;

#[macro_use]
extern crate recorder;

mod dynamic;
mod logger;
mod statik;

use recorder::{Error, Result};

const USAGE: &'static str = "
Usage: recorder <command> [options]

Commands:
    dynamic        Record dynamic characteristics.
    static         Record static characteristics.
";

fn main() {
    logger::setup();
    start().unwrap_or_else(|error| fail(error));
}

fn start() -> Result<()> {
    let arguments = ok!(arguments::parse(std::env::args()));
    if arguments.orphans.len() != 1 {
        help(USAGE);
    }
    match &arguments.orphans[0][..] {
        "dynamic" => dynamic::execute(&arguments),
        "static" => statik::execute(&arguments),
        _ => raise!("the command is unknown"),
    }
}

fn help(message: &str) -> ! {
    println!("{}", message.trim());
    std::process::exit(0);
}

fn fail(error: Error) -> ! {
    use std::io::{stderr, Write};
    stderr().write_all(format!("Error: {}.\n", &*error).as_bytes()).unwrap();
    std::process::exit(1);
}
