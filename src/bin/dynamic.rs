use arguments::Arguments;
use std::path::PathBuf;

use recorder::server::Server;
use recorder::storage::{Storage, Type, Value};
use recorder::{Result, System};

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

pub fn execute(arguments: &Arguments) -> Result<()> {
    use mcpat::Component;

    if arguments.get::<bool>("help").unwrap_or(false) {
        ::help(USAGE);
    }

    try!(System::setup(arguments));

    let mut server = try!(Server::connect(arguments));
    let storage = try!(Storage::open(arguments, &[("time", Type::Float),
                                                  ("component_id", Type::Integer),
                                                  ("dynamic_power", Type::Float)]));
    let mut writer = try!(storage.writer());

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
        let processor = try!(system.compute());

        let mut component_id = 0;

        macro_rules! write(
            ($components:expr) => (
                for component in $components {
                    try!(writer.write(&[Value::Float(time),
                                        Value::Integer(component_id),
                                        Value::Float(component.dynamic_power())]));
                    component_id += 1;
                }
            );
        );

        write!(processor.cores());
        write!(processor.l3s());
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
