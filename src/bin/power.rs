use bullet::{Database, Options, Result, Server, System};
use std::path::{Path, PathBuf};

const HALT_MESSAGE: &'static str = "bullet:halt";

const USAGE: &'static str = "
Usage: bullet power [options]

Options:
    -s, --server   HOST:PORT   Redis server (default 127.0.0.0:6379).
    -q, --queue    NAME        Queue for distributing jobs (default bullet).
    -c, --caching              Enable caching of McPAT optimization results.
    -d, --database PATH        SQLite3 database (default bullet.sqlite3).
    -t, --table    NAME        Table name for dumping results (default bullet).
    -h, --help                 Display this message.
";

pub fn execute(options: &Options) -> Result<()> {
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

    if options.get::<bool>("h").or_else(|| options.get::<bool>("help")).unwrap_or(false) {
        ::usage(USAGE);
    }

    let mut server = try!(Server::connect(options));
    let mut database = try!(Database::open(options));

    let mut prepared = false;
    let mut names = vec![];

    loop {
        let message = ok!(server.fetch());

        match &message[..] {
            HALT_MESSAGE => break,
            _ => {},
        }

        let config = PathBuf::from(message);
        let time = try!(derive_time(&config));
        let system = try!(System::load(&config));

        if !prepared {
            let (cores, l3s) = (system.cores(), system.l3s());
            names = generate_names(&[(&["core#_dynamic", "core#_leakage"], cores),
                                     (&["l3#_dynamic", "l3#_leakage"], l3s)]);
            try!(System::prepare(options));
            try!(database.prepare(&names));
            prepared = true;
        }

        let mut recorder = try!(database.recorder(&names));
        let mut power = Vec::with_capacity(recorder.len());

        let processor = try!(system.compute());
        push!(power, processor.cores());
        push!(power, processor.l3s());

        if power.len() != recorder.len() {
            raise!("encoundered a dimensionality mismatch");
        }

        try!(recorder.write(time, &power));
    }

    Ok(())
}

fn generate_names(templates: &[(&[&str], usize)]) -> Vec<String> {
    let mut names = vec![];
    for &(variants, count) in templates.iter() {
        let variants = variants.iter().map(|variant| {
            let i = variant.find('#').unwrap();
            (&variant[0..i], &variant[(i + 1)..])
        }).collect::<Vec<_>>();
        for i in 0..count {
            for &(prefix, suffix) in variants.iter() {
                names.push(format!("{}{}{}", prefix, i, suffix));
            }
        }
    }
    names
}

// Format: power-{start time}-{end time}-{elapsed time}.xml
fn derive_time(path: &Path) -> Result<u64> {
    macro_rules! bad(
        () => (raise!("encountered a malformed file path"));
    );
    let mut name = match path.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => name,
            _ => bad!(),
        },
        _ => bad!(),
    };
    if !name.starts_with("power-") || !name.ends_with(".xml") {
        bad!();
    }
    name = &name[6..(name.len() - 4)];
    let i = match name.find('-') {
        Some(i) => i + 1,
        _ => bad!(),
    };
    let j = match name.rfind('-') {
        Some(j) => j,
        _ => bad!(),
    };
    name = &name[i..j];
    match name.parse::<u64>() {
        Ok(time) => Ok(time),
        _ => bad!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate_names() {
        let names = super::generate_names(&[(&["a#b", "c#d"], 2), (&["e#f", "g#h"], 1)]);
        assert_eq!(&names[..], &["a0b", "c0d", "a1b", "c1d", "e0f", "g0h"]);
    }
}
