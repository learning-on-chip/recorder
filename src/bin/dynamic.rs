use arguments::Options;
use std::path::PathBuf;

use recorder::{Result, System};
use recorder::database::{ColumnKind, ColumnValue, Database};
use recorder::server::Server;

use support;

const MESSAGE_PREFIX: &'static str = "recorder:";
const HALT_MESSAGE: &'static str = "halt";

const USAGE: &'static str = "
Usage: recorder dynamic [options]

Options:
    --queue <name>           Queue for distributing jobs (required).
    --caching                Enable caching of McPAT optimization results.
    --server <host>:<port>   Redis server [default: 127.0.0.0:6379].

    --database <path>        SQLite3 database (required).
    --table <name>           Table for storing results (required).

    --help                   Display this message.
";

pub fn execute(options: &Options) -> Result<()> {
    use mcpat::Component;

    if options.get::<bool>("help").unwrap_or(false) {
        ::help(USAGE);
    }

    try!(System::setup(options));

    let mut server = try!(Server::connect(options));
    let database = try!(Database::open(options));
    let mut recorder = None;

    loop {
        let message = ok!(server.receive());
        if !message.starts_with(MESSAGE_PREFIX) {
            raise!("received a malformed message");
        }

        let message = &message[MESSAGE_PREFIX.len()..];
        if message == HALT_MESSAGE {
            break;
        }

        let (time, config) = try!(decode(message));
        let system = try!(System::open(&config));

        let recorder = match recorder {
            Some(ref mut recorder) => recorder,
            _ => {
                let (cores, l3s) = (system.cores(), system.l3s());
                let names = support::generate(&[(&["core#_dynamic_power"], cores),
                                                (&["l3#_dynamic_power"], l3s)]);

                let mut columns = vec![];
                columns.push((ColumnKind::Float, "time"));
                for name in names.iter() {
                    columns.push((ColumnKind::Float, name));
                }

                recorder = Some(try!(database.record(&columns)));
                recorder.as_mut().unwrap()
            },
        };

        let processor = try!(system.compute());

        macro_rules! push(
            ($columns:expr, $items:expr) => ({
                for item in $items {
                    $columns.push(ColumnValue::Float(item.dynamic_power()));
                }
            });
        );

        let mut columns = vec![ColumnValue::Float(time)];
        push!(columns, processor.cores());
        push!(columns, processor.l3s());

        try!(recorder.write(&columns));
    }

    Ok(())
}

fn decode(message: &str) -> Result<(f64, PathBuf)> {
    match message.find(';') {
        Some(i) => Ok((ok!((&message[..i]).parse::<f64>()), PathBuf::from(&message[(i + 1)..]))),
        _ => raise!("received a malformed message"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn decode() {
        match super::decode("1.2e3;/foo/bar") {
            Ok((time, path)) => {
                assert_eq!(time, 1200f64);
                assert_eq!(path, PathBuf::from("/foo/bar"));
            },
            _ => assert!(false),
        }
    }
}
