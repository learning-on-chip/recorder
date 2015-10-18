/// An address composed of a host name and a port.
pub struct Address(pub String, pub usize);

impl Address {
    /// Parse an address.
    pub fn parse(string: &str) -> Option<Address> {
        let chunks = string.split(':').collect::<Vec<_>>();
        match chunks.len() {
            2 => match chunks[1].parse::<usize>() {
                Ok(port) => Some(Address(chunks[0].to_string(), port)),
                _ => None,
            },
            _ => None,
        }
    }
}
