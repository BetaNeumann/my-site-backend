use serde::{Serialize, Deserialize};
use crate::api::v1::endpoints::prelude::*;
use crate::database::{Record, TableName};


const PUBLIC_TABLES: [TableName; 3] = [
    e::Article::TABLE_NAME,
    e::ArticleContent::TABLE_NAME,
    e::Language::TABLE_NAME
];


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    table: String,
    total: u64,
    first_id: String,
    last_id: String
}


#[debug_handler]
pub async fn get(Path(table_name): Path<String>) -> Result<Json<Metadata>, AppError> {
    if !PUBLIC_TABLES.contains(&table_name.as_str()) {
        return Err(app_err!(Validation, "Table '{table_name}' is not public"));
    }

    let metadata: Option<Metadata> = DB
        .query(queries::METADATA)
        .bind(("table_name", table_name.clone()))
        .await?
        .take(1)?;

    metadata
        .map(Json)
        .ok_or(app_err!(NotFound, "Table '{table_name}' not found"))
}
