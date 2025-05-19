mod v1;

use axum::{Router, serve, response::Redirect};
use tokio::net::TcpListener;


pub async fn init() -> std::io::Result<()> {
    let router = Router::new()
        .route("/", axum::routing::get(Redirect::to("/hello-world")))
        .route("/hello-world", axum::routing::get(async || { "Hello World!" }))
        .nest("/api/v1", v1::router());

    let listener = TcpListener::bind("localhost:3000").await?;

    serve(listener, router).await?;

    Ok(())
}
