use ara_error::{ApiError, BoxedError};
use ara_model::db::TxError;
use serde::Serialize;
use failure::Fail;

#[derive(Debug, Fail, Serialize, ApiError)]
pub enum ServiceErrorKind {
    #[fail(display = "Validation error")]
    #[api_error(http(400))]
    ValidationError,

    #[fail(display = "Transaction failed")]
    #[api_error(http(500))]
    TxFailed,

    #[fail(display = "Internal error: {}", _0)]
    #[api_error(map_from(Error), http(500))]
    Internal(BoxedError),
}
