use arguments::Options;
use hiredis;

use {Address, Result};

pub const DEFAULT_HOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: usize = 6379;

pub struct Server {
    backend: hiredis::Context,
    queue: String,
}

impl Server {
    pub fn connect(options: &Options) -> Result<Server> {
        Ok(Server {
            backend: match options.get_ref::<String>("server").and_then(|s| Address::parse(s)) {
                Some(Address(ref host, port)) => ok!(hiredis::connect(host, port)),
                _ => ok!(hiredis::connect(DEFAULT_HOST, DEFAULT_PORT)),
            },
            queue: match options.get_ref::<String>("queue") {
                Some(queue) => queue.to_string(),
                _ => raise!("a queue name is required"),
            },
        })
    }

    pub fn receive(&mut self) -> Result<String> {
        use hiredis::Reply;
        match ok!(self.backend.command(&["BLPOP", &self.queue, "0"])) {
            Reply::Array(mut elements) => match elements.pop() {
                Some(Reply::Bulk(bytes)) => Ok(ok!(String::from_utf8(bytes))),
                _ => raise!("received an unexpected reply from the server"),
            },
            _ => raise!("received an unexpected reply from the server"),
        }
    }
}
