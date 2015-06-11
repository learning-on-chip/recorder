#[macro_use] extern crate bullet;
extern crate mcpat;

mod power;

use std::{env, process};
use std::fmt::Display;
use std::io;

use bullet::Options;

const USAGE: &'static str = "
Usage: bullet <command> [options]

Commands:
    power           Record the power consumption of a system on a chip.

Options:
    -h, --help      Display the usage information of a particular command.
";

fn main() {
    let mut arguments = env::args().skip(1);
    let command = match arguments.next() {
        Some(command) => command,
        _ => usage(USAGE),
    };
    let options = match Options::parse(arguments) {
        Ok(options) => options,
        Err(error) => fail(error),
    };
    let result = match &command[..] {
        "power" => power::execute(&options),
        _ => usage(USAGE),
    };
    match result {
        Err(error) => fail(error),
        _ => {},
    }
}

#[allow(unused_must_use)]
fn fail<T: Display>(error: T) -> ! {
    use std::io::Write;
    io::stderr().write_fmt(format_args!("Error: {}.", error));
    process::exit(1);
}

#[allow(unused_must_use)]
fn usage(message: &str) -> ! {
    use std::io::Write;
    io::stderr().write_fmt(format_args!("{}", message.trim()));
    process::exit(1);
}
