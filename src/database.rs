use sqlite;
use std::path::Path;

use Result;

macro_rules! setup_sql(
    () => (
        r#"
CREATE TABLE IF NOT EXISTS bullet (id INTEGER PRIMARY KEY AUTOINCREMENT, time INTEGER{});
CREATE INDEX IF NOT EXISTS bullet_time_index ON bullet (time);
        "#
    );
);

pub struct Database {
    backend: sqlite::Database,
}

impl Database {
    #[inline]
    pub fn open(path: &Path) -> Result<Database> {
        Ok(Database { backend: ok!(sqlite::open(path)) })
    }

    #[inline]
    pub fn setup(&self, columns: &Vec<String>) -> Result<()> {
        let mut fields = String::new();
        for ref column in columns.iter() {
            fields.push_str(&format!(", {} REAL", column));
        }
        Ok(ok!(self.backend.execute(&format!(setup_sql!(), fields), None)))
    }
}
