use ara_error::{AppError, BoxedError, HttpStatus};
use failure::Fail;
use serde::Serialize;

#[derive(Debug, Fail, Serialize, HttpStatus)]
pub enum ServiceErrorKind {
    #[fail(display = "Validation error")]
    #[http_status(400)]
    ValidationError,

    #[fail(display = "Transaction failed")]
    #[http_status(500)]
    TxFailed,
}

pub type ServiceError = AppError<ServiceErrorKind>;
