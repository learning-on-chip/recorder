use arguments::Arguments;
use std::path::Path;

use recorder::database::{Database, Type, Value};
use recorder::{Result, System};

const USAGE: &'static str = "
Usage: recorder static [options]

Options:
    --config <path>          McPAT configuration file (required).

    --database <path>        SQLite database (required).
    --table <name>           Table for storing results (required).

    --caching                Enable caching of McPAT optimization results.
    --server <host>:<port>   Redis server [default: 127.0.0.0:6379].

    --help                   Display this message.
";

pub fn execute(arguments: &Arguments) -> Result<()> {
    use mcpat::Component;

    if arguments.get::<bool>("help").unwrap_or(false) {
        println!("{}", USAGE.trim());
        return Ok(());
    }

    try!(System::setup(arguments));

    let mut database = try!(Database::open(arguments, &[
        ("component_id", Type::Integer),
        ("name", Type::String),
        ("area", Type::Float),
        ("leakage_power", Type::Float),
    ]));

    let system = match arguments.get::<String>("config") {
        Some(ref config) => try!(System::open(Path::new(config))),
        _ => raise!("a configuration file of McPAT is required"),
    };
    let processor = try!(system.compute());

    let mut component_id = 0;

    macro_rules! write(
        ($components:expr, $kind:expr) => (
            for (i, component) in $components.enumerate() {
                try!(database.write(&[
                    Value::Integer(component_id),
                    Value::String(format!("{}{}", $kind, i)),
                    Value::Float(component.area()),
                    Value::Float(component.leakage_power()),
                ]));
                component_id += 1;
            }
        );
    );

    write!(processor.cores(), "core");
    write!(processor.l3s(), "l3");

    Ok(())
}
