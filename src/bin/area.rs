use std::path::PathBuf;

use bullet::{Options, Result, System};
use bullet::database::{ColumnKind, ColumnValue, Database};

use support;

const USAGE: &'static str = "
Usage: bullet area [options]

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

    macro_rules! push(
        ($columns:expr, $items:expr) => ({
            for item in $items {
                $columns.push(ColumnValue::Float(item.area()));
            }
        });
    );

    if options.get::<bool>("help").unwrap_or(false) {
        ::usage(USAGE);
    }

    try!(System::setup(options));

    let system = match options.get::<PathBuf>("config") {
        Some(ref config) => try!(System::open(config)),
        _ => raise!("a configuration file of McPAT is required"),
    };

    let database = try!(Database::open(options));
    let mut recorder = {
        let (cores, l3s) = (system.cores(), system.l3s());
        let names = support::generate(&[(&["core#"], cores), (&["l3#"], l3s)]);

        let mut columns: Vec<(ColumnKind, &str)> = vec![];
        for name in names.iter() {
            columns.push((ColumnKind::Float, name));
        }

        try!(database.record(&columns))
    };

    let processor = try!(system.compute());

    let mut columns = vec![];
    push!(columns, processor.cores());
    push!(columns, processor.l3s());

    try!(recorder.write(&columns));

    Ok(())
}
