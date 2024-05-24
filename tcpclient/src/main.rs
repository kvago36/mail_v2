use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

use std::io;

use std::error::Error;

enum State {
    Init,
    Hi,
    Mail,
    Recipient,
    Data,
    Send,
    End,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:4000").await?;
    let mut state = State::Init;

    loop {
        stream.readable().await?;

        let mut buf = [0; 4096];

        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if let Ok(data) = String::from_utf8(buf[..n].to_vec()) {
                    println!("Message from sever: {}", data)
                }

                match state {
                    State::Init => {
                        state = State::Hi;
                        stream.write_all(b"HELO sender.example.com").await?;
                    },
                    State::Hi => {
                        state = State::Mail;
                        stream.write_all(b"MAIL FROM: <sender@example.com>").await?;
                    },
                    State::Mail => {
                        state = State::Recipient;
                        stream.write_all(b"RCPT TO: <user@yandex.ru>").await?;
                    },
                    State::Recipient => {
                        state = State::Data;
                        stream.write_all(b"DATA").await?;
                    },
                    State::Data => {
                        state = State::Send;
                        stream
                        .write_all(b"To: user@yandex.ru\r\nFrom: sender@example.com\r\nSubject: example of SMTP-message\r\n\r\ntext\r\ntext\r\ntext\r\n.\r\nsd\r\n").await?;
                    },
                    State::Send => {
                        state = State::End;
                        stream.write_all(b"QUIT").await?;
                        break;
                    },
                    State::End => {
                        panic!("Closed connection")
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
