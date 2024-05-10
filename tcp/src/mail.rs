use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Mail {
    headers: HashMap<String, String>,
    from: Option<String>,
    to: Vec<String>,
    body: String,
}

impl From<String> for Mail {
    fn from(req: String) -> Self {
        let mut lines = req.lines();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut to = vec![];
        let mut from = None;

        for line in lines.by_ref().take_while(|&line| !line.is_empty()) {
            let header = line.split(":").collect::<Vec<&str>>();

            if header.len() >= 2 {
                let key = header[0].trim();
                let value = header[1].trim();

                match key {
                    "From" => from = Some(value.to_string()),
                    "To" => to = value.split(",").map(|s| s.to_string()).collect(),
                    _ => {
                        headers.insert(key.to_string(), value.to_string());
                    }
                };
            }
        }

        let body = lines
            .take_while(|&line| line.to_string() != ".".to_string())
            .collect::<Vec<_>>()
            .join("\r\n");

        Mail {
            from,
            to,
            headers,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mail_into() {
        let m: Mail = "To: user@yandex.ru,test@test.ru\r\nFrom: sender@example.com\r\nSubject: example of SMTP-message\r\n\r\ntext\r\ntext\r\ntext\r\n.\r\nsd\r\n".to_string().into();
        let mut headers = HashMap::new();
        headers.insert("Subject".into(), "example of SMTP-message".into());
        assert_eq!(headers, m.headers);
        assert_eq!(Some("sender@example.com".to_string()), m.from);
        assert_eq!(vec!("user@yandex.ru", "test@test.ru"), m.to);
        assert_eq!("text\r\ntext\r\ntext", m.body);
    }
}
