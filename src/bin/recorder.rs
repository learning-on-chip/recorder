extern crate arguments;
extern crate log;
extern crate mcpat;
extern crate term;

#[macro_use] extern crate recorder;

mod dynamic;
mod logger;
mod statik;

use recorder::Result;

use logger::Logger;

const USAGE: &'static str = "
Usage: recorder <command> [options]

Commands:
    dynamic        Record dynamic characteristics.
    static         Record static characteristics.
";

#[allow(unused_must_use)]
fn main() {
    if let Err(error) = start() {
        use std::io::Write;
        if let Some(mut output) = term::stderr() {
            output.fg(term::color::RED);
            output.write_fmt(format_args!("Error: {}.\n", error));
        }
        std::process::exit(1);
    }
}

fn start() -> Result<()> {
    let arguments = ok!(arguments::parse(std::env::args()));
    if arguments.orphans.len() != 1 {
        println!("{}", USAGE.trim());
        return Ok(());
    }
    Logger::install();
    match &arguments.orphans[0][..] {
        "dynamic" => dynamic::execute(&arguments),
        "static" => statik::execute(&arguments),
        _ => raise!("the command is unknown"),
    }
}
