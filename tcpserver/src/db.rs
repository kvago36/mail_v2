use libsql::{Connection, Builder, Error};

pub struct DB {
  connection: Option<Connection>
}

impl DB {
    fn new() -> Self {
      DB { connection: None }
    }

    pub async fn connect(&mut self, url: String, token: String) -> Result<(), Error> {
      let db = Builder::new_remote(url, token).build().await?;
      let conn = db.connect().unwrap();
      
      self.connection = Some(conn);
    }
}