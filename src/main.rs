use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

mod codec;
mod ftp;

use ftp::*;
use codec::*;

fn handle_connection(mut stream: TcpStream) {
  let reply = Reply::new(StatusCode::ServiceReadyForNewUser, "Welcome to custom FTP Server!");

  match stream.write(&Encoder::encode(reply).unwrap()) {
    Ok(_) => {},
    Err(e) => println!("Connection error: {}", e)
  }

  // let mut packet: [u8; 32] = [0; 32];

  // match stream.read(&mut packet) {
  //   Ok(_) => println!("Packet: {:?}", String::from_utf8_lossy(&packet[..])),
  //   Err(e) => println!("Error: {}", e)
  // }
  // stream.write(&bytes).unwrap();
  // println!("Packet: {:?}", String::from_utf8_lossy(&packet[..]));
}

fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080")?;

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }

  Ok(())
}
