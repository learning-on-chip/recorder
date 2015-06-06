use mcpat;
use std::path::Path;

use {Options, Result, server};

pub struct System {
    backend: mcpat::System,
}

impl System {
    #[inline]
    pub fn load(path: &Path) -> Result<System> {
        let backend = ok!(mcpat::open(path));
        {
            let system = backend.raw();
            if system.number_of_L2s > 0 && system.Private_L2 == 0 {
                raise!("shared L2 caches are currently not supported");
            }
        }
        Ok(System { backend: backend })
    }

    pub fn prepare(options: &Options) -> Result<()> {
        mcpat::set_optimzed_for_clock_rate(true);

        if options.caching.unwrap_or(false) {
            match options.server {
                Some((ref host, port)) => ok!(mcpat::caching::activate(host, port)),
                None => ok!(mcpat::caching::activate(server::DEFAULT_HOST, server::DEFAULT_PORT)),
            }
        }

        Ok(())
    }

    #[inline]
    pub fn processor<'l>(&'l self) -> Result<mcpat::Processor<'l>> {
        Ok(ok!(self.backend.processor()))
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
