use ara_error::{AppError, HttpStatus};
use failure::Fail;
use serde::Serialize;

#[derive(Debug, Fail, Serialize, HttpStatus)]
pub enum ServiceErrorKind {
    #[fail(display = "Validation error")]
    #[http_status(400)]
    ValidationError,
}

pub type ServiceError = AppError<ServiceErrorKind>;
