use std::io::{Error, Write};

use crate::ftp::{Reply, ReplyMessage, Request};

pub struct Encoder {}

impl Encoder {
    pub fn encode(reply: Reply) -> Result<Vec<u8>, Error> {
        let mut vec = Vec::new();

        match reply.message {
            ReplyMessage::Is(message) => write!(vec, "{} {}\r\n", reply.code as u32, message)?,
            _ => write!(vec, "{}\r\n", reply.code as u32)?,
        }

        Ok(vec)
    }
}

// struct Decoder {}

// impl Decoder {
//   pub fn decode() {}
// }

#[cfg(test)]
mod tests {
    use crate::ftp::*;
    use crate::Encoder;

    #[test]
    fn encode() {
        let result = Encoder::encode(Reply::new(
            StatusCode::ServiceReadyForNewUser,
            ReplyMessage::Is(String::from("Test")),
        ));
        assert!(result.is_ok());
    }
}
