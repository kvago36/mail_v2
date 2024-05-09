use std::io::BufRead;

use tcp::mail::Mail;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use libsql::Builder;

use std::cell::RefCell;
use std::rc::Rc;
use std::env;

mod server;
mod state;
mod db;

use state::Post;
use tcp::request::TcpRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

    let db = Builder::new_remote(url, token).build().await?;
    let conn = db.connect().unwrap();

    conn.execute("SELECT * FROM mails", ()).await.unwrap();

    let listener = TcpListener::bind("localhost:4000").await?;
    // let mut isMail = false;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let smtp = server::Server::new(socket);
            smtp.serve().await;

            // let mut state = Post::new(&mut socket);
            // let mut buf = [0; 1024];
            // let buf_reader = BufReader::new(&mut stream);

            // state.greeting().await;

            // socket.write(b"220 mxfront7.mail.yandex.net").await;

            // In a loop, read data from the socket and write the data back.
            // loop {
            //     let n = match socket.read(&mut buf).await {
            //         // socket closed
            //         Ok(n) if n == 0 => return,
            //         Ok(n) => n,
            //         Err(e) => {
            //             eprintln!("failed to read from socket; err = {:?}", e);
            //             return;
            //         }
            //     };

            //     if let Ok(data) = String::from_utf8(buf[..n].to_vec()) {
            //         // println!("{}", data);
            //         if isMail {
            //             isMail = false;

            //             let mail = Mail::from(data);
            //             println!("{:?}", mail);
            //         } else {
            //             let req = TcpRequest::from(data);

            //             // println!(
            //             //     "command: {:?} body: {:?}",
            //             //     req.command,
            //             //     req.payload.map_or(String::from(""), |x| x.into_string())
            //             // );

            //             match req.command {
            //                 tcp::request::Command::DATA => {
            //                     isMail = true;
            //                 }
            //                 _ => {}
            //             }

            //             // state.handle_message(req).await;
            //         }
            //     }

            //     // Write the data back
            //     // if let Err(e) = socket.write_all(&buf[0..n]).await {
            //     //     eprintln!("failed to write to socket; err = {:?}", e);
            //     //     return;
            //     // }
            // }
        });
    }
}
