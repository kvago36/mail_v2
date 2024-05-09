use std::error::Error;

use tcp::mail::Mail;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;

use tcp::request::TcpRequest;

use crate::state::{Post, READY_TO_RECIEVE, END};

pub struct Server {
    state: Post,
    stream: TcpStream,
    data: bool
}

impl Server {
    pub fn new(stream: TcpStream) -> Self {
        Server {
            state: Post::new(),
            stream: stream,
            data: false
        }
    }

    fn generate_random_string(length: usize) -> String {
        let mut rng = rand::thread_rng();
        let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let random_string: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();
        random_string
    }

    pub async fn serve(mut self) -> Result<(), Box<dyn Error>> {
        self.greet().await?;

        let mut buf = vec![0; 65536];
        loop {
            let n = self.stream.read(&mut buf).await?;

            if n == 0 {
                break;
            }

            let msg = String::from_utf8(buf[..n].to_vec())?;

            println!("raw request from client: {}", msg);

            if self.data == true {
                let mail = Mail::from(msg);

                self.data = false;

                println!("server get email{:?}", mail);

                let random = Server::generate_random_string(8);
                let ds = format!("250 Ok: queued as {}", random);
                

                println!("server responded to email{:?}", ds);

                self.stream.write_all(ds.as_bytes()).await?;
            } else {
                let request = TcpRequest::from(msg);

                println!("server parser request to: {:?}", request.command);

                match self.state.handle_message(request) {
                    Ok((code, response)) => {
                        // println!("SERVER READY FOR RESPONSE!");

                        if code == END {
                            // println!("SERVER READY FOR RESPONSE!");

                            break;
                        }

                        if code == READY_TO_RECIEVE {
                            // println!("!!!!!!!!");
                            self.data = true
                        }

                        // println!("server responded for request: {}", response);

                        self.stream.write_all(response.as_bytes()).await?;
                    }
                    Err(e) => {
                        // println!("ERROR! {}", e);
                        self.stream.write_all(b"500").await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Sends the initial SMTP greeting
    async fn greet(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.greeting();

        self.stream
            .write_all(b"200 mxfront7.mail.yandex.net")
            .await
            .map_err(|e| e.into())
    }
}
