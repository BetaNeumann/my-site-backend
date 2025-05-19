use axum::{Router, routing::get};
use crate::api::v1;


pub fn router() -> Router {
    Router::new()
        .route("/metadata/{table_name}", get(v1::metadata::get))
        .route("/language", get(v1::language::get))
        .route("/language/{id}", get(v1::language::get_by_id))
        .route("/ex", get(ex::get))
}


mod ex {
    use axum::{extract::Query, Json};
    use crate::database::{Record, RecordId};
    use crate::error::AppError;
    use crate::api::v1::pagination::{Pagination, PaginationResult};

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Example {
        id: RecordId<Self>,
        name: String,
        #[serde(rename = "type")]
        type_: String,
        in_stock: bool,
        price: f32
    }

    impl Record for Example {
        const TABLE_NAME: &str = "example";

        fn id(&self) -> &surrealdb::RecordIdKey {
            self.id.key()
        }
    }

    #[axum::debug_handler]
    pub async fn get(Query(query): Query<Pagination>) -> Result<Json<PaginationResult<Example>>, AppError> {
        let result: PaginationResult<Example> = query.query_for(&crate::database::DB).await?;
        Ok(Json(result))
    }
}
