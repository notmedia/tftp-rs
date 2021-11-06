use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

enum Command {
  USER,
  QUIT,
  PORT,
  TYPE,
  MODE,
  STRU,
  RETR,
  STOR,
  NOOP
}

fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080")?;

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }

  Ok(())
}

fn handle_connection(mut stream: TcpStream) {
  let mut packet: [u8; 100] = [0; 100];

  stream.read(&mut packet).unwrap();

  println!("Packet: {:?}", String::from_utf8_lossy(&packet[..]));
}
