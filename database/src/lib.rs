pub mod traits;
pub mod records;

use std::ops::Deref;
use std::sync::OnceLock;

use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::{Client, Ws};


pub struct Connection(OnceLock<Surreal<Client>>);

impl Connection {
    pub const fn new() -> Self {
        return Connection(OnceLock::new())
    }

    pub fn set(&self, conn: Surreal<Client>) -> Result<(), Surreal<Client>> {
        if self.0.get().is_some() {
            return  Err(conn);
        }
        self.0.set(conn)?;
        return Ok(());
    }
}

impl Deref for Connection {
    type Target = Surreal<Client>;

    fn deref(&self) -> &Self::Target {
        self.0.get().expect("Database connection not initialized")
    }
}

pub async fn init() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("localhost:8000").await.expect("Failed to connect to DB");
    db.signin(Root {username: "root", password: "root"}).await.expect("Failed to authenticate to DB");
    db.use_ns("test").use_db("test").await.expect("Failed to defined namespace and database");

    return db;
}
