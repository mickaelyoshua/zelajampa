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
