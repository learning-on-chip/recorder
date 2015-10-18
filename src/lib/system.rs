use arguments::Arguments;
use mcpat;
use std::path::Path;

use {Address, Result};

/// A McPAT system.
pub struct System {
    backend: mcpat::System,
}

impl System {
    /// Open a system.
    #[inline]
    pub fn open<T: AsRef<Path>>(path: T) -> Result<System> {
        let backend = ok!(mcpat::open(path));
        {
            let system = backend.raw();
            if system.number_of_L2s > 0 && system.Private_L2 == 0 {
                raise!("shared L2 caches are currently not supported");
            }
        }
        Ok(System { backend: backend })
    }

    /// Configure global parameters.
    pub fn setup(arguments: &Arguments) -> Result<()> {
        use server::{DEFAULT_HOST, DEFAULT_PORT};

        mcpat::optimze_for_clock_rate(true);

        if arguments.get::<bool>("caching").unwrap_or(false) {
            match arguments.get::<String>("server").and_then(|s| Address::parse(&s)) {
                Some(Address(ref host, port)) => ok!(mcpat::caching::activate(host, port)),
                _ => ok!(mcpat::caching::activate(DEFAULT_HOST, DEFAULT_PORT)),
            }
        }

        Ok(())
    }

    /// Perform the computation of area and power.
    #[inline]
    pub fn compute<'l>(&'l self) -> Result<mcpat::Processor<'l>> {
        Ok(ok!(self.backend.compute()))
    }

    /// Return the number of cores.
    pub fn cores(&self) -> usize {
        let system = self.backend.raw();
        if system.homogeneous_cores != 0 { 1 } else {
            system.number_of_cores as usize
        }
    }

    /// Return the number of L3 caches.
    pub fn l3s(&self) -> usize {
        let system = self.backend.raw();
        if system.homogeneous_L3s != 0 { 1 } else {
            system.number_of_L3s as usize
        }
    }
}
