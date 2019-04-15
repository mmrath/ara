use std::fmt::{self, Display};
use std::io::Cursor;

use log::error;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;
use failure::Fail;

#[derive(Debug)]
pub struct HttpError {
    body: JsonValue,
    status: Status,
    source: Option<failure::Error>,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.body.0, self.status)
    }
}

impl<T> From<T> for HttpError
where
    T: ara_error::HttpResponse + Fail,
{
    fn from(e: T) -> Self {
        let status = Status::from_code(e.status()).unwrap_or(Status::InternalServerError);
        if status == Status::InternalServerError {
            error!("An internal error occurred {:?}", e);
        }
        HttpError {
            body: JsonValue(e.body()),
            status,
            source: Some(failure::Error::from(e)),
        }
    }
}

impl<'r> Responder<'r> for HttpError {
    fn respond_to(self, _req: &Request<'_>) -> Result<Response<'r>, Status> {
        let body = self.body;

        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.0.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

#[allow(dead_code)]
pub(crate) fn ok() -> HttpError {
    HttpError {
        body: json!(null),
        status: Status::Ok,
        source: None,
    }
}

#[allow(dead_code)]
pub(crate) fn internal_server_error(e: failure::Error) -> HttpError {
    HttpError {
        body: json!(null),
        status: Status::InternalServerError,
        source: Some(e),
    }
}
