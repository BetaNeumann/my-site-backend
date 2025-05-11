mod database;

use std::sync::LazyLock;
use std::collections::HashMap;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root};

use crate::database::{Connection, entities as e};

static DB: LazyLock<Connection> = LazyLock::new(Connection::init);


#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:8000").await?;
    DB.signin(Root {username: "root", password: "root"}).await?;
    DB.use_ns("test").use_db("test").await?;

    // let mut result = DB.query("SELECT * FROM language").await?;

    // let languages: HashMap<String, e::Language> = result
    //     .take::<Vec<e::Language>>(0)?
    //     .into_iter()
    //     .map(|l| (l.description.clone(), l))
    //     .collect();

    let article: Option<e::Article> = DB
        .create(e::Article::new("my-article".into()))
        .await?;

    let article = article.unwrap();

    // let article_content = e::ArticleContent::new(
    //     article.id.clone(),
    //     languages["english"].id.clone(),
    //     "My Article".into(),
    //     "yay".into(),
    //     false
    // );

    dbg!(article);
    Ok(())
}
