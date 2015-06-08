use sqlite;
use std::path::Path;

use {Options, Result};

pub const DEFAULT_FILE: &'static str = "bullet.sqlite3";
pub const DEFAULT_TABLE: &'static str = "bullet";

macro_rules! prepare_sql(
    ($table:expr, $fields:expr) => (
        format!(r#"
CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY AUTOINCREMENT, time INTEGER{});
CREATE INDEX IF NOT EXISTS {}_time_index ON {} (time);
        "#, $table, $fields, $table, $table)
    );
);

macro_rules! statement_sql(
    ($table:expr, $fields:expr, $values:expr) => (
        format!(r#"
INSERT INTO {} (time{}) VALUES (?{});
        "#, $table, $fields, $values)
    );
);

pub struct Database<'l> {
    backend: sqlite::Database<'l>,
    table: String,
}

pub struct Recorder<'l> {
    length: usize,
    backend: sqlite::Statement<'l>,
}

impl<'l> Database<'l> {
    #[inline]
    pub fn open(options: &Options) -> Result<Database<'l>> {
        Ok(Database {
            backend: match options.database {
                Some(ref path) => ok!(sqlite::open(path)),
                None => ok!(sqlite::open(&Path::new(DEFAULT_FILE))),
            },
            table: match options.table {
                Some(ref table) => table.to_string(),
                None => String::from(DEFAULT_TABLE),
            },
        })
    }

    pub fn prepare(&mut self, columns: &Vec<String>) -> Result<()> {
        let mut fields = String::new();
        for ref name in columns.iter() {
            fields.push_str(&format!(", {} REAL", name));
        }
        Ok(ok!(self.backend.execute(&prepare_sql!(&self.table, &fields))))
    }

    #[inline]
    pub fn recorder(&'l self, columns: &[String]) -> Result<Recorder<'l>> {
        Recorder::new(self, columns)
    }
}

impl<'l> Recorder<'l> {
    pub fn new(database: &'l Database, columns: &[String]) -> Result<Recorder<'l>> {
        let mut fields = String::new();
        let mut values = String::new();
        for ref name in columns.iter() {
            fields.push_str(&format!(", {}", name));
            values.push_str(", ?");
        }

        let backend = ok!(database.backend.prepare(&statement_sql!(&database.table,
                                                                   fields, values)));

        Ok(Recorder {
            length: columns.len(),
            backend: backend,
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn write(&mut self, time: u64, values: &[f64]) -> Result<()> {
        use sqlite::Binding::{Float, Integer};
        use sqlite::ResultCode::Done;

        if self.length != values.len() {
            raise!("encoundered a dimensionality mistmatch");
        }

        let mut bindings = Vec::with_capacity(1 + self.length);

        bindings.push(Integer(1, time as i64));
        for i in 0..self.length {
            bindings.push(Float(i + 2, values[i]));
        }

        ok!(self.backend.reset());
        ok!(self.backend.bind(&bindings));

        match self.backend.step() {
            Done => Ok(()),
            _ => raise!("cannot write data into the database"),
        }
    }
}
