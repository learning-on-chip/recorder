use mcpat;

use std::fs;
use std::path::Path;

use {Database, Options, Result};

pub struct Experiment {
    system: mcpat::System,
    database: Database,
}

impl Experiment {
    pub fn new(options: Options) -> Result<Experiment> {
        let Options { config, database, .. } = options;

        let config = match config {
            Some(config) => config,
            None => raise!("a configuration file is required"),
        };
        if !exists(&config) {
            raise!("the configuration file does not exist");
        }

        let database = match database {
            Some(database) => database,
            None => raise!("a database file is required"),
        };

        let system = ok!(mcpat::open(&config));
        let database = try!(Database::open(&database));

        {
            let system = system.raw();
            assert!(system.Private_L2 != 0);
        }

        Ok(Experiment { system: system, database: database })
    }

    pub fn run(&mut self) -> Result<()> {
        use mcpat::Component;

        let processor = ok!(self.system.processor());
        let mut power = vec![];
        for core in processor.cores() {
            power.push(core.power());
        }
        for l3 in processor.l3s() {
            power.push(l3.power());
        }

        Ok(())
    }

    pub fn setup(&mut self) -> Result<()> {
        try!(self.database.setup());
        Ok(())
    }
}

fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}
