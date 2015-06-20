use arguments::Options;
use sqlite;
use std::thread;

use Result;

pub const FAIL_SLEEP_MS: u32 = 50;
pub const FAIL_ATTEMPTS: usize = 10;

macro_rules! create_sql(
    ($table:expr, $fields:expr) => (
        format!(r#"
CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY AUTOINCREMENT, {});
        "#, $table, $fields)
    );
);

macro_rules! insert_sql(
    ($table:expr, $fields:expr, $values:expr) => (
        format!(r#"
INSERT INTO {} ({}) VALUES ({});
        "#, $table, $fields, $values)
    );
);

pub struct Database<'l> {
    backend: sqlite::Database<'l>,
    table: String,
}

#[derive(Clone, Copy)]
pub enum ColumnKind {
    Float,
    Text,
}

#[derive(Clone, Copy)]
pub enum ColumnValue<'l> {
    Float(f64),
    Text(&'l str),
}

pub struct Recorder<'l> {
    backend: sqlite::Statement<'l>,
}

impl<'l> Database<'l> {
    pub fn open(options: &Options) -> Result<Database<'l>> {
        let mut backend = match options.get_ref::<String>("database") {
            Some(database) => ok!(sqlite::open(database)),
            _ => raise!("a database is required"),
        };
        ok!(backend.set_busy_handler(|_| {
            thread::sleep_ms(FAIL_SLEEP_MS);
            true
        }));
        Ok(Database {
            backend: backend,
            table: match options.get_ref::<String>("table") {
                Some(table) => table.to_string(),
                _ => raise!("a table name is required"),
            },
        })
    }

    pub fn record(&'l self, columns: &[(ColumnKind, &str)]) -> Result<Recorder<'l>> {
        let mut fields = String::new();
        for &(kind, name) in columns.iter() {
            if !fields.is_empty() {
                fields.push_str(", ");
            }
            fields.push_str(name);
            match kind {
                ColumnKind::Float => fields.push_str(" REAL"),
                ColumnKind::Text => fields.push_str(" TEXT"),
            }
        }
        ok!(self.backend.execute(&create_sql!(&self.table, &fields)));

        Recorder::new(self, columns)
    }
}

impl<'l> Recorder<'l> {
    pub fn new(database: &'l Database, columns: &[(ColumnKind, &str)]) -> Result<Recorder<'l>> {
        let mut fields = String::new();
        let mut values = String::new();
        for &(_, name) in columns.iter() {
            if !fields.is_empty() {
                fields.push_str(", ");
                values.push_str(", ");
            }
            fields.push_str(name);
            values.push_str("?");
        }

        Ok(Recorder {
            backend: ok!(database.backend.prepare(&insert_sql!(&database.table, fields, values))),
        })
    }

    pub fn write<'c>(&mut self, columns: &[ColumnValue<'c>]) -> Result<()> {
        use sqlite::State;

        let mut success = false;
        for _ in 0..FAIL_ATTEMPTS {
            ok!(self.backend.reset());
            for (i, &value) in columns.iter().enumerate() {
                match value {
                    ColumnValue::Float(value) => ok!(self.backend.bind(i + 1, value)),
                    ColumnValue::Text(value) => ok!(self.backend.bind(i + 1, value)),
                }
            }
            match self.backend.step() {
                Ok(state) if state == State::Done => {
                    success = true;
                    break;
                },
                _ => thread::sleep_ms(FAIL_SLEEP_MS),
            }
        }
        if !success {
            raise!("cannot write into the database");
        }

        Ok(())
    }
}
