use arguments::Options;
use std::path::Path;

use recorder::{Result, System};
use recorder::database::{ColumnKind, ColumnValue, Database};

use support;

const USAGE: &'static str = "
Usage: recorder static [options]

Options:
    --config <path>          McPAT configuration file (required).

    --database <path>        SQLite3 database (required).
    --table <name>           Table for storing results (required).

    --caching                Enable caching of McPAT optimization results.
    --server <host>:<port>   Redis server [default: 127.0.0.0:6379].

    --help                   Display this message.
";

pub fn execute(options: &Options) -> Result<()> {
    use mcpat::Component;

    if options.get::<bool>("help").unwrap_or(false) {
        ::help(USAGE);
    }

    try!(System::setup(options));

    let system = match options.get_ref::<String>("config") {
        Some(config) => try!(System::open(Path::new(config))),
        _ => raise!("a configuration file of McPAT is required"),
    };

    let database = try!(Database::open(options));
    let mut recorder = try!(database.record(&[(ColumnKind::Text, "name"),
                                              (ColumnKind::Float, "value")]));

    let (cores, l3s) = (system.cores(), system.l3s());
    let names = support::generate(&[(&["core#_area", "core#_leakage_power"], cores),
                                    (&["l3#_area", "l3#_leakage_power"], l3s)]);

    let processor = try!(system.compute());

    macro_rules! write(
        ($recorder:expr, $items:expr, $names:expr, $counter:expr) => (
            for item in $items {
                try!($recorder.write(&[ColumnValue::Text(&names[$counter]),
                                       ColumnValue::Float(item.area())]));
                $counter += 1;
                try!($recorder.write(&[ColumnValue::Text(&names[$counter]),
                                       ColumnValue::Float(item.leakage_power())]));
                $counter += 1;
            }
        );
    );

    let mut k = 0;
    write!(recorder, processor.cores(), names, k);
    write!(recorder, processor.l3s(), names, k);

    Ok(())
}
