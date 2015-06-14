use options::Options;

use Result;

pub fn parse<I>(stream: I) -> Result<Options> where I: Iterator<Item=String> {
    macro_rules! truth(
        ($result:expr) => (if !$result {
            raise!("the arguments are invalid");
        });
    );

    let mut options = Options::new();
    let mut previous: Option<String> = None;

    for chunk in stream {
        if chunk.starts_with("--") {
            if let Some(name) = previous {
                options.set(&name, true);
            }
            truth!(chunk.len() > 2);
            previous = Some(String::from(&chunk[2..]));
        } else if let Some(name) = previous {
            options.set(&name, String::from(chunk));
            previous = None;
        } else {
            truth!(false);
        }
    }
    if let Some(name) = previous {
        options.set(&name, true);
    }

    Ok(options)
}
