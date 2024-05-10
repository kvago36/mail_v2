use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("localhost:4000").await?;
    // loop {
    let mut buf = [0; 1024];

    let n = stream.read(&mut buf).await?;

    if let Ok(data) = String::from_utf8(buf[..n].to_vec()) {
        println!("{}", data)
    }

    // Write some data.
    stream.write_all(b"HELO sender.example.com").await?;

    sleep(Duration::from_millis(100)).await;

    // Write some data.
    stream.write_all(b"MAIL FROM: <sender@example.com>").await?;

    sleep(Duration::from_millis(100)).await;

    // Write some data.
    stream.write_all(b"RCPT TO: <user@yandex.ru>").await?;

    sleep(Duration::from_millis(100)).await;

    // Write some data.
    stream.write_all(b"DATA").await?;

    sleep(Duration::from_millis(100)).await;

    stream
        .write_all(b"To: user@yandex.ru\r\nFrom: sender@example.com\r\nSubject: example of SMTP-message\r\n\r\ntext\r\ntext\r\ntext\r\n.\r\nsd\r\n").await?;

    sleep(Duration::from_millis(100)).await;

    stream.write_all(b"QUIT").await?;
    // }

    Ok(())
}
