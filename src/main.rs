extern crate bullet;

use std::{env, process};
use std::fmt::Display;

use bullet::Options;

const USAGE: &'static str = "
Usage: bullet [options]

Options:
    -c FILE, --config   FILE       Configuration file in XML (required).
    -d FILE, --database FILE       Database file in SQLite3 (required).
    -r,      --caching  HOST:PORT  Enable caching of optimization results.
    -p,      --prepare             Set up the database and exit.

    -h,      --help                Display this message.
";

fn main() {
    match bullet::process(options()) {
        Err(error) => fail(error),
        _ => {},
    }
}

fn options() -> Options {
    macro_rules! truth(
        ($result:expr) => (if !$result { usage(); });
    );

    let mut options = Options::new();
    let mut previous: Option<String> = None;
    for argument in env::args().skip(1) {
        match &argument[..] {
            "-h" | "--help" => usage(),
            _ => {},
        }
        if argument.starts_with("--") {
            if argument.len() < 3 {
                usage();
            }
            if let Some(name) = previous {
                truth!(options.set_flag(name, true));
            }
            previous = Some(String::from(&argument[2..]));
        } else if argument.starts_with("-") {
            if argument.len() != 2 {
                usage();
            }
            if let Some(name) = previous {
                truth!(options.set_flag(name, true));
            }
            previous = Some(String::from(&argument[1..]));
        } else if let Some(name) = previous {
            truth!(options.set_value(name, argument));
            previous = None;
        } else {
            usage();
        }
    }
    if let Some(name) = previous {
        truth!(options.set_flag(name, true));
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
