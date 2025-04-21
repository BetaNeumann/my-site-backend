use sqlx::SqlitePool;
use crate::database::Conn;


pub async fn connect() -> Result<Conn, sqlx::Error> {
    let url = std::env::var(&"DATABASE_URL").expect("Define database url");
    return SqlitePool::connect(&url).await;
}
