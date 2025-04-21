use sqlx::FromRow;
use time::OffsetDateTime;


#[derive(Debug, FromRow)]
pub struct Visitor {
    pub id: i64,
    pub name: String,
    pub created_at: OffsetDateTime,
}
