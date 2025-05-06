use time::UtcDateTime;
use database::{Connection, records::Visitor, traits::Record};


static DB: Connection = Connection::new();

#[tokio::main]
async fn main() {
    DB.set(database::init().await).unwrap();

    let result: Vec<Visitor> = DB
        .insert(Visitor::TABLE_NAME)
        .content(Visitor {
            name: "Bruno".to_owned(),
            created_at: UtcDateTime::now()
        })
        .await
        .unwrap();

    dbg!(result);
}
