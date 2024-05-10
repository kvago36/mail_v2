use std::error::Error;

use mongodb::{Collection};
use tcp::mail::Mail;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use tcp::request::TcpRequest;

use crate::state::{Post, END, READY_TO_RECIEVE};

pub struct Server {
    state: Post,
    store: Collection<Mail>,
    stream: TcpStream,
    data: bool,
}

impl Server {
    pub fn new(stream: TcpStream, store: Collection<Mail>) -> Self {
        Server {
            state: Post::new(),
            stream,
            store,
            data: false,
        }
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

                let result = self.store.insert_one(mail, None).await;

                match result {
                    Ok(insert_one) => {
                        let inserted = format!("250 Ok: queued as {}", insert_one.inserted_id);
                        self.stream.write_all(inserted.as_bytes()).await?;
                    }
                    Err(e) => {
                        println!("{}", e);
                        self.stream.write_all(b"500 Error").await?;
                    }
                }

                self.data = false;
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
