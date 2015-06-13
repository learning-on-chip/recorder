use std::path::{Path, PathBuf};

use bullet::{Options, Result, System};
use bullet::database::{ColumnKind, ColumnValue, Database};
use bullet::server::Server;

use support;

const HALT_MESSAGE: &'static str = "bullet:halt";

const USAGE: &'static str = "
Usage: bullet power [options]

Options:
    --server   HOST:PORT     Redis server (default 127.0.0.0:6379).
    --queue    NAME          Queue for distributing jobs (default bullet).
    --caching                Enable caching of McPAT optimization results.

    --database PATH          SQLite3 database (default bullet.sqlite3).
    --table    NAME          Table name for dumping results (default bullet).

    --help                   Display this message.
";

pub fn execute(options: &Options) -> Result<()> {
    use mcpat::Component;

    macro_rules! push(
        ($columns:expr, $items:expr) => ({
            for item in $items {
                $columns.push(ColumnValue::Float(item.dynamic_power()));
                $columns.push(ColumnValue::Float(item.leakage_power()));
            }
        });
    );

    if options.get::<bool>("help").unwrap_or(false) {
        ::usage(USAGE);
    }

    try!(System::setup(options));

    let mut server = try!(Server::connect(options));
    let database = try!(Database::open(options));
    let mut recorder = None;

    loop {
        let message = ok!(server.receive());

        match &message[..] {
            HALT_MESSAGE => break,
            _ => {},
        }

        let config = PathBuf::from(message);
        let time = try!(derive_time(&config));
        let system = try!(System::open(&config));

        let recorder = match recorder {
            Some(ref mut recorder) => recorder,
            _ => {
                let (cores, l3s) = (system.cores(), system.l3s());
                let names = support::generate(&[(&["core#_dynamic", "core#_leakage"], cores),
                                                (&["l3#_dynamic", "l3#_leakage"], l3s)]);

                let mut columns = vec![];
                columns.push((ColumnKind::Integer, "time"));
                for name in names.iter() {
                    columns.push((ColumnKind::Float, name));
                }

                recorder = Some(try!(database.record(&columns)));
                recorder.as_mut().unwrap()
            }
        };

        let processor = try!(system.compute());

        let mut columns = vec![ColumnValue::Integer(time as i64)];
        push!(columns, processor.cores());
        push!(columns, processor.l3s());

        try!(recorder.write(&columns));
    }

    Ok(())
}

// Format: power-{time}-{whatever}.xml
pub fn derive_time(path: &Path) -> Result<u64> {
    macro_rules! bad(
        () => (raise!("encountered a malformed file path"));
    );
    let mut name = match path.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => name,
            _ => bad!(),
        },
        _ => bad!(),
    };
    if !name.starts_with("power-") || !name.ends_with(".xml") {
        bad!();
    }
    name = &name[6..];
    let i = match name.find('-') {
        Some(i) => i,
        _ => bad!(),
    };
    name = &name[..i];
    match name.parse::<u64>() {
        Ok(time) => Ok(time),
        _ => bad!(),
    }
}
