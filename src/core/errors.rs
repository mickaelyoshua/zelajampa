use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Database error occurred")]
    DatabaseError(#[from] sqlx::Error),

    // Fallback genérico para impedir vazamentos indesejados
    #[error("Internal Server Error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(_) => {
                // Não repassamos a string crua do banco para o frontend!
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Database Error".to_string(),
                )
            }
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "System Error".to_string(),
            ),
        };

        (status, error_message).into_response()
    }
}
