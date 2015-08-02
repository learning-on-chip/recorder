use arguments::Arguments;
use database::Database;
use database::driver::{Driver, SQLite, Statement};
use database::query::{CreateTable, Insert};

pub use database::{Type, Value};

use Result;

pub struct Storage<'l> {
    table: String,
    columns: Vec<(String, Type)>,
    backend: Database<SQLite<'l>>,
}

pub struct Writer<'l> {
    backend: <SQLite<'l> as Driver>::Statement,
}

impl<'l> Storage<'l> {
    pub fn open(arguments: &Arguments, columns: &[(&str, Type)]) -> Result<Storage<'l>> {
        let table = match arguments.get::<String>("table") {
            Some(table) => table,
            _ => raise!("a table name is required"),
        };

        let backend = match arguments.get::<String>("database") {
            Some(ref database) => ok!(Database::open(database)),
            _ => raise!("a database is required"),
        };

        let query = CreateTable::new().name(&table).if_not_exists();
        let query = columns.iter().fold(query, |query, &(name, kind)| {
            query.column(|column| column.name(name).kind(kind))
        });
        ok!(backend.execute(query));

        let columns = columns.iter().map(|&(name, kind)| (name.to_string(), kind)).collect();

        Ok(Storage { table: table, columns: columns, backend: backend })
    }

    pub fn writer(&self) -> Result<Writer<'l>> {
        let query = Insert::new().table(&self.table);
        let query = self.columns.iter().fold(query, |query, &(ref name, _)| query.column(name));
        Ok(Writer { backend: ok!(self.backend.prepare(query)) })
    }
}

impl<'l> Writer<'l> {
    #[inline]
    pub fn write(&mut self, values: &[Value]) -> Result<()> {
        ok!(self.backend.execute(values));
        Ok(())
    }
}
