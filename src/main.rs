extern crate bullet;

use std::{env, process, mem};
use std::default::Default;

const USAGE: &'static str = "
Usage: bullet [options] <config>

Options:
    -h, --help   Display this message.
";

#[derive(Debug, Default)]
struct Arguments {
    config: String,
}

fn main() {
    let _args = setup();
    bullet::say("Hello!");
}

fn setup() -> Arguments {
    let mut arguments = Arguments::new();
    let mut name: Option<&str> = None;
    for argument in env::args() {
        match &argument[..] {
            "-h" | "--help" => usage(),
            "-c" | "--config" => name = Some("config"),
            _ => {
                if name.is_some() {
                    arguments.set(mem::replace(&mut name, None).unwrap(), argument);
                } else {
                    usage();
                }
            },
        }
    }
    arguments
}

fn usage() -> ! {
    fail(USAGE.trim());
}

fn fail(message: &str) -> ! {
    println!("{}", message);
    process::exit(1);
}

impl Arguments {
    #[inline]
    fn new() -> Arguments {
        Default::default()
    }

    fn set(&mut self, name: &str, value: String) {
        match name {
            "config" => self.config = value,
            _ => usage(),
        }
    }
}
