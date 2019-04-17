use ara_error::{ApiError,BoxedError};
use serde::Serialize;
use failure::{Fail};

#[derive(Debug, Fail, Serialize, ApiError)]
pub enum ServiceErrorKind {
    #[fail(display = "Validation error")]
    #[api_error(http(400))]
    ValidationError,

    #[fail(display = "{}", _0)]
    #[api_error(map_from(Error), http(500))]
    Internal(BoxedError),
}
