#[derive(Debug, PartialEq)]
pub struct TcpResponse<'a> {
    code: &'a str,
    body: String,
}

impl<'a> TcpResponse<'a> {
    fn new(code: &'a str, body: String) -> Self {
        TcpResponse { code, body }
    }
}

impl<'a> From<&'a str> for TcpResponse<'a> {
    fn from(res: &'a str) -> Self {
        let mut words = res.split_whitespace();

        let code = words.next().unwrap_or_default();
        let body = words.collect::<Vec<&str>>().join(" ");

        TcpResponse { code, body }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_into() {
        let c: TcpResponse = "220 mail.company.tld ESMTP is glad to see you!".into();
        assert_eq!("220", c.code);
        assert_eq!("mail.company.tld ESMTP is glad to see you!", c.body);
    }
}
