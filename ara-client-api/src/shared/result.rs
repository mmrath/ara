use crate::shared::error::HttpError;
use rocket_contrib::json::Json;
use serde::Serialize;

pub type JsonResult<T> = Result<Json<T>, HttpError>;

pub trait HttpResultExt<T, E> {
    fn into_json(self) -> Result<Json<T>, HttpError>
    where
        T: Serialize,
        E: Into<HttpError>;
}

impl<T, E> HttpResultExt<T, E> for Result<T, E>
where
    T: Serialize,
    E: Into<HttpError>,
{
    fn into_json(self) -> Result<Json<T>, HttpError> {
        self.map(Json).map_err(Into::into)
    }
}
