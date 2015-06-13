use std::path::PathBuf;

use bullet::{Options, Result, System};
use bullet::database::{ColumnKind, ColumnValue, Database};

use support;

const USAGE: &'static str = "
Usage: bullet static [options]

Options:
    --config   PATH          McPAT configuration file (required).

    --database PATH          SQLite3 database (default bullet.sqlite3).
    --table    NAME          Table name for dumping results (default bullet).

    --caching                Enable caching of McPAT optimization results.
    --server   HOST:PORT     Redis server (default 127.0.0.0:6379).

    --help                   Display this message.
";

pub fn execute(options: &Options) -> Result<()> {
    use mcpat::Component;

    if options.get::<bool>("help").unwrap_or(false) {
        ::usage(USAGE);
    }

    try!(System::setup(options));

    let system = match options.get::<PathBuf>("config") {
        Some(ref config) => try!(System::open(config)),
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
