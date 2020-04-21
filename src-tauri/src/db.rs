use crate::local;
use rusqlite::{params, Connection};

pub struct DB {
  pub conn: Connection,
}

impl DB {
  pub fn new() -> Self {
    let name = std::env::var("DATABASE_NAME").unwrap();
    let path = local::get_path(local::Directory::Data, Some(&name));
    let conn = Connection::open(path).expect("failed to open sqlite connection.");
    conn
      .execute(
        "CREATE TABLE IF NOT EXISTS projects (
        id INTEGER PRIMARYKEY,
        name TEXT
      )",
        params![],
      )
      .expect("failed to create storage table");

    Self { conn }
  }

  //   pub fn insert(&self) -> Result<usize, rusqlite::Error> {
  //     self.conn.execute(
  //                   "
  //                     INSERT INTO 1? (name, temp, popularity)
  //                     VALUES('San Jose', '18', 'low'),
  //                     ('Domingo', '29', 'high'),
  //                     ('Paparacci', '-20', 'medium');
  //               ", params![])
  //   }
}
