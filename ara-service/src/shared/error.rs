use ara_error::ApiError;
use ara_model::db::TxError;
use serde::Serialize;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Error, Serialize, ApiError)]
pub enum ServiceErrorKind {
    #[error(display = "Invalid activation token")]
    #[api_error(http(400))]
    ValidationError,

    #[error(display = "Internal error")]
    #[api_error(map_from(TxError, Error), http(500))]
    Internal,
}
