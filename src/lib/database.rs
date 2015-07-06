use arguments::Arguments;
use sqlite;
use std::thread;

use Result;

pub const FAIL_SLEEP_MS: u32 = 50;
pub const FAIL_ATTEMPTS: usize = 10;

pub struct Database<'l> {
    table: String,
    columns: Vec<(String, ColumnKind)>,
    backend: sqlite::Connection<'l>,
}

#[derive(Clone, Copy)]
pub enum ColumnKind {
    Float,
    Integer,
    Text,
}

#[derive(Clone, Copy)]
pub enum ColumnValue<'l> {
    Float(f64),
    Integer(i64),
    Text(&'l str),
}

pub struct Statement<'l> {
    backend: sqlite::Statement<'l>,
}

impl<'l> Database<'l> {
    pub fn open(arguments: &Arguments, columns: &[(&str, ColumnKind)]) -> Result<Database<'l>> {
        let table = match arguments.get::<String>("table") {
            Some(table) => table,
            _ => raise!("a table name is required"),
        };

        let columns = columns.iter().map(|&(name, kind)| (name.to_string(), kind))
                                    .collect::<Vec<_>>();

        let mut backend = match arguments.get::<String>("database") {
            Some(ref database) => ok!(sqlite::open(database)),
            _ => raise!("a database is required"),
        };
        ok!(backend.set_busy_handler(|_| {
            thread::sleep_ms(FAIL_SLEEP_MS);
            true
        }));

        let mut fields = String::new();
        for &(ref name, kind) in columns.iter() {
            if !fields.is_empty() {
                fields.push_str(", ");
            }
            fields.push_str(name);
            match kind {
                ColumnKind::Float => fields.push_str(" REAL"),
                ColumnKind::Integer => fields.push_str(" INTEGER"),
                ColumnKind::Text => fields.push_str(" TEXT"),
            }
        }
        ok!(backend.execute(&format!("
            CREATE TABLE IF NOT EXISTS {} ({});
        ", &table, &fields)));

        Ok(Database { table: table, columns: columns, backend: backend })
    }

    #[inline]
    pub fn prepare(&'l self) -> Result<Statement<'l>> {
        Statement::new(self)
    }
}

impl<'l> Statement<'l> {
    pub fn new(database: &'l Database) -> Result<Statement<'l>> {
        let mut fields = String::new();
        let mut values = String::new();
        for &(ref name, _) in database.columns.iter() {
            if !fields.is_empty() {
                fields.push_str(", ");
                values.push_str(", ");
            }
            fields.push_str(name);
            values.push_str("?");
        }

        Ok(Statement {
            backend: ok!(database.backend.prepare(&format!("
                INSERT INTO {} ({}) VALUES ({});
            ", &database.table, fields, values))),
        })
    }

    pub fn write<'c>(&mut self, columns: &[ColumnValue<'c>]) -> Result<()> {
        use sqlite::State;

        let mut success = false;
        for _ in 0..FAIL_ATTEMPTS {
            ok!(self.backend.reset());
            for (mut i, &value) in columns.iter().enumerate() {
                i += 1;
                match value {
                    ColumnValue::Float(value) => ok!(self.backend.bind(i, value)),
                    ColumnValue::Integer(value) => ok!(self.backend.bind(i, value)),
                    ColumnValue::Text(value) => ok!(self.backend.bind(i, value)),
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
