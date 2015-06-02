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

    pub fn setup(&self) -> Result<()> {
        let system = self.system.raw();

        let cores = if system.homogeneous_cores != 0 { 1 } else {
            system.number_of_cores as usize
        };
        let l3s = if system.homogeneous_L3s != 0 { 1 } else {
            system.number_of_L3s as usize
        };

        let names = generate(&[("core#_dynamic", cores), ("core#_leakage", cores),
                               ("l3#_dynamic", l3s), ("l3#_leakage", l3s)]);

        try!(self.database.setup(&names));

        Ok(())
    }
}

fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}

fn generate(pairs: &[(&str, usize)]) -> Vec<String> {
    let mut names = vec![];
    for &(name, count) in pairs.iter() {
        let (prefix, suffix) = {
            let i = name.find('#').unwrap();
            (&name[0..i], &name[(i + 1)..])
        };
        for i in 0..count {
            names.push(format!("{}{}{}", prefix, i, suffix));
        }
    }
    names
}
