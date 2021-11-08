pub enum Command {
    USER,
    QUIT,
    PORT,
    TYPE,
    MODE,
    STRU,
    RETR,
    STOR,
    NOOP,
}

pub enum StatusCode {
    ServiceReadyForNewUser = 220,
}

pub enum ReplyMessage {
    None,
    Is(String),
}

pub struct Reply {
    pub code: StatusCode,
    pub message: ReplyMessage,
}

impl Reply {
    pub fn new(code: StatusCode, message: ReplyMessage) -> Self {
        Reply { code, message }
    }
}

pub struct Request {
    pub command: Command,
}

impl Request {
    pub fn new(command: &str) -> Self {
        Request {
            command: Command::NOOP,
        }
    }
}
