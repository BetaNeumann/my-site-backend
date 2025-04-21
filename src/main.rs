mod structs;
mod database;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    _ = dotenv::dotenv();

    let conn = database::connect().await?;
    let visitor = database::operations::create_visitor(&conn).await?;
    println!("{visitor:?}");

    return Ok(())
}
