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

pub struct Statement<'l> {
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

    pub fn statement<'l>(&'l self, columns: &Vec<String>) -> Result<Statement<'l>> {
        let mut fields = String::new();
        let mut values = String::new();
        for ref name in columns.iter() {
            fields.push_str(&format!(", {}", name));
            values.push_str(", ?");
        }
        let backend = ok!(self.backend.statement(&format!(statement_sql!(), fields, values)));
        Ok(Statement { backend: backend })
    }
}
