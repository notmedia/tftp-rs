use std::io::{Write};

use crate::ftp::{Reply, ReplyMessage};

pub struct Encoder {}

impl Encoder {
    pub fn encode(reply: Reply) -> Result<Vec<u8>, std::io::Error> {
        let mut vec = Vec::new();

        match reply.message {
            ReplyMessage::Is(message) => write!(vec, "{} {}\r\n", reply.code as u32, message)?,
            _ => write!(vec, "{}\r\n", reply.code as u32)?,
        }

        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::ftp::*;
    use crate::{Encoder};

    #[test]
    fn encode() {
        let result = Encoder::encode(Reply::new(
            StatusCode::ServiceReadyForNewUser,
            ReplyMessage::Is(String::from("Test")),
        ));
        assert!(result.is_ok());
    }
}
