use mongodb::Client;
use tokio::net::TcpListener;

mod server;
mod state;

use tcp::mail::Mail;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    // let token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("mails");
    let collection = database.collection::<Mail>("mails");

    let listener = TcpListener::bind("localhost:4000").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let coll_ref = collection.clone();

        tokio::spawn(async move {
            let smtp = server::Server::new(socket, coll_ref);
            smtp.serve().await;
        });
    }
}
