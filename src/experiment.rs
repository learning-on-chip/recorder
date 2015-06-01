use mcpat;

use std::fs;
use std::path::Path;

use {Options, Result};

pub struct Experiment<'l> {
    system: mcpat::System<'l>,
}

impl<'l> Experiment<'l> {
    pub fn new(options: Options) -> Result<Experiment<'l>> {
        let Options { config, .. } = options;

        let config = match config {
            Some(config) => config,
            None => raise!("a configuration file is required"),
        };
        if !exists(&config) {
            raise!("the configuration file does not exist");
        }

        Ok(Experiment {
            system: ok!(mcpat::open(&config)),
        })
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
}

fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}
