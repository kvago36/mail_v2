use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Mail {
    headers: HashMap<String, String>,
    body: String,
}

impl From<String> for Mail {
    fn from(req: String) -> Self {
        let mut lines = req.lines();
        let mut headers: HashMap<String, String> = HashMap::new();

        for line in lines.by_ref().take_while(|&line| !line.is_empty()) {
            let header = line.split(":").collect::<Vec<&str>>();

            if header.len() >= 2 {
                let key = header[0].trim().to_lowercase().to_string();
                let value = header[1].trim().to_string();
                headers.insert(key, value);
            }
        }

        let content = lines
            .take_while(|&line| line.to_string() != ".".to_string())
            .collect::<Vec<_>>()
            .join("\r\n");

        Mail {
            headers: headers,
            body: content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mail_into() {
        let m: Mail = "To: user@yandex.ru\r\nFrom: sender@example.com\r\nSubject: example of SMTP-message\r\n\r\ntext\r\ntext\r\ntext\r\n.\r\nsd\r\n".to_string().into();
        let mut headers = HashMap::new();
        headers.insert("to".into(), "user@yandex.ru".into());
        headers.insert("from".into(), "sender@example.com".into());
        headers.insert("subject".into(), "example of SMTP-message".into());
        assert_eq!(headers, m.headers);
        assert_eq!("text\r\ntext\r\ntext", m.body);
    }
}
