use sqlite;
use std::path::Path;

use Result;

macro_rules! prepare_sql(
    () => (
        r#"
CREATE TABLE IF NOT EXISTS bullet (id INTEGER PRIMARY KEY AUTOINCREMENT, time INTEGER{});
CREATE INDEX IF NOT EXISTS bullet_time_index ON bullet (time);
        "#
    );
);

macro_rules! statement_sql(
    () => (
        r#"
INSERT INTO bullet (time{}) VALUES (?{});
        "#
    );
);

pub struct Database {
    backend: sqlite::Database,
}

pub struct Recorder<'l> {
    length: usize,
    backend: sqlite::Statement<'l>,
}

impl Database {
    #[inline]
    pub fn open(path: &Path) -> Result<Database> {
        let backend = ok!(sqlite::open(path));
        Ok(Database { backend: backend })
    }

    pub fn prepare(&self, columns: &Vec<String>) -> Result<()> {
        let mut fields = String::new();
        for ref name in columns.iter() {
            fields.push_str(&format!(", {} REAL", name));
        }
        Ok(ok!(self.backend.execute(&format!(prepare_sql!(), fields), None)))
    }

    #[inline]
    pub fn recorder<'l>(&'l self, columns: &[String]) -> Result<Recorder<'l>> {
        Recorder::new(&self.backend, columns)
    }
}

impl<'l> Recorder<'l> {
    pub fn new(backend: &'l sqlite::Database, columns: &[String]) -> Result<Recorder<'l>> {
        let mut fields = String::new();
        let mut values = String::new();
        for ref name in columns.iter() {
            fields.push_str(&format!(", {}", name));
            values.push_str(", ?");
        }

        let backend = ok!(backend.statement(&format!(statement_sql!(), fields, values)));

        Ok(Recorder {
            length: columns.len(),
            backend: backend,
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn write(&mut self, time: i64, values: &[f64]) -> Result<()> {
        use sqlite::Binding::{Float, Integer};
        use sqlite::ResultCode::Done;

        if self.length != values.len() {
            raise!("encoundered a dimensionality mistmatch");
        }

        let mut bindings = Vec::with_capacity(1 + self.length);

        bindings.push(Integer(1, time));
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
