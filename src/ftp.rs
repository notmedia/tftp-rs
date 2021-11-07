pub enum Command {
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

pub enum StatusCode {
  ServiceReadyForNewUser = 220,
}

pub struct Reply {
  pub code: StatusCode,
  pub message: String,
}

impl Reply {
  pub fn new(code: StatusCode, message: &str) -> Self {
    Reply { code, message: message.to_string() }
  }
}

pub struct Request {
  pub command: Command,
  pub payload: String,
}

impl Request {
  pub fn new(command: &str, payload: &str) -> Self {
    Request { command: Command::NOOP, payload: payload.to_string() }
  }
}
