//! Redis server.

use arguments::Arguments;
use hiredis;

use {Address, Result};

/// The default Redis host name.
pub const DEFAULT_HOST: &'static str = "127.0.0.1";

/// The default Redis port number.
pub const DEFAULT_PORT: usize = 6379;

/// A Redis server.
pub struct Server {
    backend: hiredis::Context,
    queue: String,
}

impl Server {
    /// Establish a connection to a server.
    pub fn connect(arguments: &Arguments) -> Result<Server> {
        Ok(Server {
            backend: match arguments.get::<String>("server").and_then(|s| Address::parse(&s)) {
                Some(Address(ref host, port)) => ok!(hiredis::connect(host, port)),
                _ => ok!(hiredis::connect(DEFAULT_HOST, DEFAULT_PORT)),
            },
            queue: match arguments.get::<String>("queue") {
                Some(queue) => queue,
                _ => raise!("a queue name is required"),
            },
        })
    }

    /// Fetch a message from the server.
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
