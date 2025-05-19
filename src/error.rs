use axum::{Json, http::StatusCode, response::{Response, IntoResponse}};
use thiserror::Error;
use serde::Serialize;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database - {0}")]
    Db(#[from] surrealdb::Error),

    #[error("Not Found - {0}")]
    NotFound(String),

    #[error("Invalid Input - {0}")]
    Validation(String),

    #[error("{1} - {2}")]
    Custom(StatusCode, String, String)
}


#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB_ERROR"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            AppError::Custom(code, _, _) => (code.into(), "UNEXPECTED")
        };

        let body = ErrorBody {
            code,
            message: self.to_string(),
        };

        (status, Json(body)).into_response()
    }
}

#[macro_export]
macro_rules! app_err {
    ($code:expr, $title:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        $crate::error::AppError::Custom(
            $code,
            $title.into(),
            format!($fmt $(, $args)*),
        )
    };

    ($variant:ident, $fmt:expr $(, $args:expr)* $(,)?) => {
        $crate::error::AppError::$variant(format!($fmt $(, $args)*))
    };
}
