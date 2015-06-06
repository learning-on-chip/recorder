use mcpat::{Component, Power};
use std::path::PathBuf;

use {Database, Options, Result, Server, System};

const HALT_MESSAGE: &'static str = "bullet:halt";

pub struct Worker {
    server: Server,
    database: Database,
    options: Options,
}

impl Worker {
    pub fn new(options: Options) -> Result<Worker> {
        Ok(Worker {
            server: try!(Server::connect(&options)),
            database: try!(Database::open(&options)),
            options: options,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        macro_rules! push(
            ($power:expr, $items:expr) => ({
                for item in $items {
                    let Power { dynamic, leakage } = item.power();
                    $power.push(dynamic);
                    $power.push(leakage);
                }
            });
        );

        let mut prepared = false;
        let mut names = vec![];

        loop {
            let message = ok!(self.server.fetch());

            match &message[..] {
                HALT_MESSAGE => break,
                _ => {},
            }

            let config = PathBuf::from(message);
            let system = try!(System::load(&config));

            if !prepared {
                let (cores, l3s) = (system.cores(), system.l3s());
                names = generate(&[("core#_dynamic", cores), ("core#_leakage", cores),
                                 ("l3#_dynamic", l3s), ("l3#_leakage", l3s)]);
                try!(System::prepare(&self.options));
                try!(self.database.prepare(&names));
                prepared = true;
            }

            let mut recorder = try!(self.database.recorder(&names));
            let mut power = Vec::with_capacity(recorder.len());

            let processor = try!(system.processor());
            push!(power, processor.cores());
            push!(power, processor.l3s());

            if power.len() != recorder.len() {
                raise!("encoundered a dimensionality mismatch");
            }

            try!(recorder.write(0, &power));
        }

        Ok(())
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
