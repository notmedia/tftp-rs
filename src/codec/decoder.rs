use std::io::{Error};

use crate::ftp::{Request, Command};

pub struct Decoder {}

impl Decoder {
  pub fn decode() -> Result<Request, Error> {
    Ok(Request {
      command: Command::NOOP,
      payload: String::from("Test payload")
    })
  }
}

#[cfg(test)]
mod tests {
    use crate::{Decoder};

    #[test]
    fn decode() {
      let result = Decoder::decode();
    }
}
