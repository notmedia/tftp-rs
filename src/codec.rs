use std::io::{Write, Error};

use crate::ftp::{Reply, Request};

pub struct Encoder {}

impl Encoder {
  pub fn encode(reply: Reply) -> Result<Vec<u8>, Error> {
    let mut vec = Vec::new();

    if reply.message.is_empty() {
      write!(vec, "{}\r\n", reply.code as u32)?;
    } else {
      write!(vec, "{} {}\r\n", reply.code as u32, reply.message)?;
    }

    Ok(vec)
  }
}

// struct Decoder {}

// impl Decoder {
//   pub fn decode() {}
// }