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
            if system.number_of_L2s > 0 && system.Private_L2 == 0 {
                raise!("shared L2 caches are currently not supported");
            }
        }

        Ok(Experiment { system: system, database: database })
    }

    pub fn run(&mut self) -> Result<()> {
        use mcpat::{Component, Power};

        macro_rules! push(
            ($power:expr, $items:expr) => ({
                for item in $items {
                    let Power { dynamic, leakage } = item.power();
                    $power.push(dynamic);
                    $power.push(leakage);
                }
            });
        );

        let mut recorder = try!(self.database.recorder(&self.names()));
        let mut power = Vec::with_capacity(recorder.len());

        let processor = ok!(self.system.processor());
        push!(power, processor.cores());
        push!(power, processor.l3s());

        if power.len() != recorder.len() {
            raise!("encoundered a dimensionality mismatch");
        }

        try!(recorder.write(0, &power));

        Ok(())
    }

    pub fn prepare(&self) -> Result<()> {
        try!(self.database.prepare(&self.names()));
        Ok(())
    }

    fn names(&self) -> Vec<String> {
        let (cores, l3s) = (self.cores(), self.l3s());
        generate(&[("core#_dynamic", cores), ("core#_leakage", cores),
                   ("l3#_dynamic", l3s), ("l3#_leakage", l3s)])
    }

    fn cores(&self) -> usize {
        let system = self.system.raw();
        if system.homogeneous_cores != 0 { 1 } else {
            system.number_of_cores as usize
        }
    }

    fn l3s(&self) -> usize {
        let system = self.system.raw();
        if system.homogeneous_L3s != 0 { 1 } else {
            system.number_of_L3s as usize
        }
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
