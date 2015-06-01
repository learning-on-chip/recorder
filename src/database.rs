use sqlite;
use std::path::Path;

use Result;

pub struct Database {
    backend: sqlite::Database,
}

impl Database {
    pub fn open(path: &Path) -> Result<Database> {
        Ok(Database { backend: ok!(sqlite::open(path)) })
    }
}
