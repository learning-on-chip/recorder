use arguments::Arguments;
use database::Database;
use database::driver::sqlite;

pub use database::{Type, Value};

use Result;

pub struct Storage<'l> {
    table: String,
    columns: Vec<(String, Type)>,
    backend: Database<sqlite::Driver<'l>>,
}

pub struct Writer<'l> {
    backend: sqlite::Prepared<'l>,
}

impl<'l> Storage<'l> {
    pub fn open(arguments: &Arguments, columns: &[(&str, Type)]) -> Result<Storage<'l>> {
        use database::language::*;

        let table = match arguments.get::<String>("table") {
            Some(table) => table,
            _ => raise!("a table name is required"),
        };

        let backend = match arguments.get::<String>("database") {
            Some(ref database) => ok!(Database::open(database)),
            _ => raise!("a database is required"),
        };

        let statement = create_table().name(&table).if_not_exists();
        let statement = columns.iter().fold(statement, |statement, &(name, kind)| {
            statement.column(column().name(name).kind(kind))
        });
        ok!(backend.execute(statement));

        let columns = columns.iter().map(|&(name, kind)| (name.to_string(), kind)).collect();

        Ok(Storage { table: table, columns: columns, backend: backend })
    }

    pub fn writer(&self) -> Result<Writer<'l>> {
        use database::language::*;

        let statement = insert_into().table(&self.table);
        let statement = self.columns.iter().fold(statement, |statement, &(ref name, _)| {
            statement.column(name)
        });
        Ok(Writer { backend: ok!(self.backend.prepare(statement)) })
    }
}

impl<'l> Writer<'l> {
    #[inline]
    pub fn write(&mut self, values: &[Value]) -> Result<()> {
        use database::driver::Prepared;

        ok!(self.backend.execute(values));
        Ok(())
    }
}
