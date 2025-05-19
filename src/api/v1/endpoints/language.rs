use crate::api::v1::endpoints::prelude::*;


#[debug_handler]
pub async fn get(Query(query): Query<Pagination>) -> Result<Json<PaginationResult<e::Language>>, AppError> {
    let result: PaginationResult<e::Language> = query.query_for(&DB).await?;
    Ok(Json(result))
}


#[debug_handler]
pub async fn get_by_id(Path(id): Path<String>) -> Result<Json<e::Language>, AppError> {
    let language: Option<e::Language> = DB.get(&id).await?;

    language
        .map(Json)
        .ok_or(AppError::NotFound(format!("Language with id '{id}' not found")))
}
