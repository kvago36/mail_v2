#[derive(Debug, PartialEq)]
pub struct Header {
    key: String,
    value: String,
}

impl Header {
    fn new(key: &str, value: &str) -> Self {
        Header {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "HELO" => Command::HELO,
            "EHLO" => Command::HELO,
            "MAIL" => Command::MAIL,
            "RCPT" => Command::RCPT,
            "DATA" => Command::DATA,
            "QUIT" => Command::QUIT,
            _ => Command::UNKNOWN,
        }
    }
}

impl From<&str> for Payload {
    fn from(s: &str) -> Self {
        let values: Vec<&str> = s.split(":").collect();

        match values.len() {
            1 => Payload::Value(s.to_string()),
            _ => Payload::KeyValue(Header::new(values[0], values[1])),
        }
    }
}

pub struct TcpRequest {
    pub command: Command,
    pub payload: Option<Payload>,
}

impl From<String> for TcpRequest {
    fn from(req: String) -> Self {
        let (command, payload) = process_req_line(req);

        TcpRequest {
            command: command,
            payload: payload,
        }
    }
}

fn process_req_line(s: String) -> (Command, Option<Payload>) {
    let mut words = s.split_whitespace();
    let command = words.next().unwrap();

    let arguments: Vec<&str> = words.collect::<Vec<_>>();

    if arguments.is_empty() {
        (command.into(), None)
    } else {
        (command.into(), Some(arguments.join("").as_str().into()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    HELO,
    MAIL,
    RCPT,
    DATA,
    QUIT,
    UNKNOWN,
}

#[derive(Debug, PartialEq)]
pub enum Payload {
    Value(String),
    KeyValue(Header),
}

impl Payload {
    pub fn into_string(self) -> String {
        match self {
            Payload::Value(str) => str,
            Payload::KeyValue(value) => format!("{}: {}", value.key, value.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_command_into() {
        let c: Command = "MAIL".into();
        assert_eq!(c, Command::MAIL);
    }
    #[test]
    fn test_payload_into() {
        let p: Payload = "key:value".into();
        let h: Header = Header::new("key", "value");
        assert_eq!(p, Payload::KeyValue(h));
    }
    #[test]
    fn test_read_tcp() {
        let s: String = String::from("HELO sender.example.com");
        let req: TcpRequest = s.into();
        assert_eq!(Command::HELO, req.command);
        assert_eq!(
            Some(Payload::Value("sender.example.com".to_string())),
            req.payload
        );
    }
}
