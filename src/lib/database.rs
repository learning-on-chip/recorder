use arguments::Arguments;
use sqlite;
use std::{mem, thread};

use Result;

pub use sql::Type;
pub use sqlite::Value;

const FAIL_SLEEP_MS: u32 = 50;
const FAIL_ATTEMPTS: usize = 10;

pub struct Database {
    #[allow(dead_code)]
    connection: sqlite::Connection,
    cursor: sqlite::Cursor<'static>,
}

impl Database {
    pub fn open(arguments: &Arguments, columns: &[(&str, Type)]) -> Result<Database> {
        use sql::prelude::*;

        let table = match arguments.get::<String>("table") {
            Some(table) => table,
            _ => raise!("a table name is required"),
        };

        let mut connection = match arguments.get::<String>("database") {
            Some(ref database) => ok!(sqlite::open(database)),
            _ => raise!("a database is required"),
        };
        ok!(connection.set_busy_handler(|_| {
            error!(target: "database", "Failed to execute a query. Trying again...");
            thread::sleep_ms(FAIL_SLEEP_MS);
            true
        }));

        let mut statement = create_table().name(&table).if_not_exists();
        statement = columns.iter().fold(statement, |statement, &(name, kind)| {
            statement.column(column().name(name).kind(kind))
        });
        ok!(connection.execute(ok!(statement.compile())));

        let mut statement = insert_into().table(&table);
        statement = columns.iter().fold(statement, |statement, &(ref name, _)| {
            statement.column(name)
        });
        let cursor = {
            let cursor = ok!(connection.prepare(ok!(statement.compile()))).cursor();
            let clone = unsafe { mem::transmute_copy(&cursor) };
            mem::forget(cursor);
            clone
        };

        Ok(Database { connection: connection, cursor: cursor })
    }

    pub fn write(&mut self, values: &[Value]) -> Result<()> {
        let mut success = false;
        for _ in 0..FAIL_ATTEMPTS {
            ok!(self.cursor.bind(values));
            if self.cursor.next().is_ok() {
                success = true;
                break;
            }
            error!(target: "database", "Failed to insert a record. Trying again...");
            thread::sleep_ms(FAIL_SLEEP_MS);
        }
        if !success {
            raise!("cannot write into the database");
        }
        Ok(())
    }
}
