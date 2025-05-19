pub mod queries;
pub mod error;
pub mod database;
pub mod api;

use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

use crate::database::DB;
use crate::api::init;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    DB.connect::<Ws>("localhost:8000").await.unwrap();
    DB.signin(Root {username: "root", password: "root"}).await.unwrap();
    DB.use_ns("test").use_db("test").await.unwrap();

    init().await.expect("Failed to setup App");
}
