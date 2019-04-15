use ara_error::{ApiError,BoxedError};
use ara_model::db::TxError;
use serde::Serialize;
use failure::{Fail, Error};

#[derive(Debug, Fail, Serialize, ApiError)]
pub enum ServiceErrorKind {
    #[fail(display = "Validation error")]
    #[api_error(http(400))]
    ValidationError,

    #[fail(display = "{}", _0)]
    #[api_error(map_from(Error), http(500))]
    Internal(BoxedError),
}

/*
impl From<Error> for ara_error::AraError<ServiceErrorKind>{
    fn from(e: Error) -> Self {
        AraError::from(e.into())
    }
}
*/