use sqlite;
use std::path::Path;

use Result;


pub struct Database {
    backend: sqlite::Database,
}

impl Database {
    #[inline]
    pub fn open(path: &Path) -> Result<Database> {
        Ok(Database { backend: ok!(sqlite::open(path)) })
    }

    #[inline]
    pub fn setup(&self) -> Result<()> {
        let sql = format!(r#"
CREATE TABLE IF NOT EXISTS bullet (
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    time INTEGER
);
CREATE INDEX IF NOT EXISTS bullet_time_index ON bullet (time);
        "#);
        Ok(ok!(self.backend.execute(sql.trim(), None)))
    }
}
