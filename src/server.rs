use hiredis;

use {Options, Result};

pub const DEFAULT_HOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: usize = 6379;

pub const DEFAULT_QUEUE: &'static str = "bullet-queue";

pub struct Server {
    backend: hiredis::Context,
}

impl Server {
    #[inline]
    pub fn connect(options: &Options) -> Result<Server> {
        Ok(Server {
            backend: match options.server {
                Some((ref host, port)) => ok!(hiredis::connect(host, port)),
                None => ok!(hiredis::connect(DEFAULT_HOST, DEFAULT_PORT)),
            },
        })
    }

    #[inline]
    pub fn fetch(&mut self) -> Result<String> {
        use hiredis::Reply;
        match ok!(self.backend.command(&["BLPOP", DEFAULT_QUEUE, "0"])) {
            Reply::Array(mut elements) => match elements.pop() {
                Some(Reply::Bulk(bytes)) => Ok(ok!(String::from_utf8(bytes))),
                _ => raise!("received an unexpected reply from the server"),
            },
            _ => raise!("received an unexpected reply from the server"),
        }
    }
}
