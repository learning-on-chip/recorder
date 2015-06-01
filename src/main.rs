extern crate bullet;

use std::{env, process};
use std::fmt::Display;

use bullet::Options;

const USAGE: &'static str = "
Usage: bullet [options]

Options:
    -c FILE, --config   FILE       Configuration file in XML (required).
    -d FILE, --database FILE       Database file in SQLite3 (required).
    -h,      --help                Display this message.
";

fn main() {
    match bullet::process(setup()) {
        Err(error) => fail(error),
        _ => {},
    }
}

fn setup() -> Options {
    let mut options = Options::new();
    let mut name: Option<&str> = None;
    for argument in env::args().skip(1) {
        match &argument[..] {
            "-h" | "--help" => usage(),
            "-c" | "--config" => name = Some("config"),
            "-d" | "--database" => name = Some("database"),
            _ => match name {
                Some(key) => {
                    if !options.set(key, argument) {
                        usage();
                    }
                    name = None;
                },
                None => usage(),
            },
        }
    }
    if name.is_some() {
        usage();
    }
    options
}

#[inline]
fn usage() -> ! {
    println!("{}", USAGE.trim());
    process::exit(1);
}

#[inline]
fn fail<T: Display>(message: T) -> ! {
    println!("Error: {}.", message);
    process::exit(1);
}
