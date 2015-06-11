use mcpat;
use std::path::Path;

use {Address, Options, Result};

pub struct System {
    backend: mcpat::System,
}

impl System {
    #[inline]
    pub fn open(path: &Path) -> Result<System> {
        let backend = ok!(mcpat::open(path));
        {
            let system = backend.raw();
            if system.number_of_L2s > 0 && system.Private_L2 == 0 {
                raise!("shared L2 caches are currently not supported");
            }
        }

        Ok(System { backend: backend })
    }

    pub fn setup(options: &Options) -> Result<()> {
        use server::{DEFAULT_HOST, DEFAULT_PORT};

        mcpat::set_optimzed_for_clock_rate(true);

        if options.get::<bool>("caching").unwrap_or(false) {
            match options.get::<Address>("server") {
                Some((ref host, port)) => ok!(mcpat::caching::activate(host, port)),
                _ => ok!(mcpat::caching::activate(DEFAULT_HOST, DEFAULT_PORT)),
            }
        }

        Ok(())
    }

    #[inline]
    pub fn compute<'l>(&'l self) -> Result<mcpat::Processor<'l>> {
        Ok(ok!(self.backend.compute()))
    }

    pub fn cores(&self) -> usize {
        let system = self.backend.raw();
        if system.homogeneous_cores != 0 { 1 } else {
            system.number_of_cores as usize
        }
    }

    pub fn l3s(&self) -> usize {
        let system = self.backend.raw();
        if system.homogeneous_L3s != 0 { 1 } else {
            system.number_of_L3s as usize
        }
    }
}
