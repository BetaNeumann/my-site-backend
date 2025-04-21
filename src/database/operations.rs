use sqlx::query_as;
use haikunator::Haikunator;
use crate::database::Conn;
use crate::structs;


pub async fn create_visitor(conn: &Conn) -> Result<structs::Visitor, sqlx::Error> {
    let haikunator = Haikunator::default();
    let visitor_name = haikunator.haikunate();

    let mut transaction = conn.begin().await?;

    let visitor = query_as!(
        structs::Visitor,
        "INSERT INTO visitor (name) VALUES ($1) RETURNING id, name, created_at;",
        visitor_name
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;
    return Ok(visitor);
}


pub async fn get_visitor(conn: &Conn, visitor_id: i64) -> Result<structs::Visitor, sqlx::Error> {
    return query_as!(
        structs::Visitor,
        "SELECT id, name, created_at FROM visitor WHERE id = $1",
        visitor_id
    )
    .fetch_one(conn)
    .await
}
