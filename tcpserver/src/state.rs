use std::error;
use std::fmt;
use tcp::request::{Command, TcpRequest};

pub static READY: &str = "220";
pub static OK: &str = "250";
pub static READY_TO_RECIEVE: &str = "354";
pub static END: &str = "221";

enum State {
    Init,
    SendHi,
    Greeted(String),
    Mail(String),
    Data,
    Recipient(Vec<String>),
    Ended,
}

pub struct Post {
    state: State,
    content: String,
}

#[derive(Debug)]
pub enum CustomError {
    UnknowCommand,
    EmptyPayloadError,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::UnknowCommand => write!(f, "Unknown command!"),
            CustomError::EmptyPayloadError => write!(f, "Payload not found!"),
        }
    }
}

impl error::Error for CustomError {}

impl Post {
    pub fn new() -> Self {
        Post {
            state: State::Init,
            content: String::from("mxfront7.mail.yandex.net"),
        }
    }

    pub fn greeting(&mut self) {
        self.state = State::SendHi;
    }

    pub fn handle_message(&mut self, req: TcpRequest) -> Result<(&str, String), CustomError> {
        if let Some(payload) = req.payload {
            match (req.command, &self.state) {
                (Command::HELO, State::SendHi) => {
                    let domain_name = payload.into_string();
                    let response_string = format!("220 {}", &domain_name);

                    self.state = State::Greeted(domain_name);

                    Ok((READY, response_string))
                }
                (Command::MAIL, State::Greeted(_)) => {
                    let mail_from_address = payload.into_string();
                    let response_string = format!("250 {} ok", &mail_from_address);

                    self.state = State::Mail(mail_from_address.clone());

                    Ok((OK, response_string))
                }
                (Command::RCPT, State::Mail(_)) => {
                    let recipient_addredd = payload.into_string();
                    let response_string = format!("250 {}", &recipient_addredd);

                    self.state = State::Recipient(vec![recipient_addredd.clone()]);

                    Ok((OK, response_string))
                }
                (Command::RCPT, State::Recipient(address)) => {
                    let recipient_addredd = payload.into_string();
                    let response_string = format!("250 {} recipient ok", &recipient_addredd);

                    address.clone().push(recipient_addredd.clone());

                    self.state = State::Recipient(address.to_vec());

                    Ok((OK, response_string))
                }
                _ => Err(CustomError::UnknowCommand),
            }
        } else {
            match req.command {
                Command::QUIT => {
                    self.state = State::Ended;

                    Ok((END, "End\n".to_string()))
                }
                Command::DATA => {
                    self.state = State::Data;
                    Ok((
                        READY_TO_RECIEVE,
                        "354 End data with <CR><LF>.<CR><LF>\n".to_string(),
                    ))
                }
                _ => Err(CustomError::EmptyPayloadError),
            }
        }
    }
}
