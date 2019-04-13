use log::error;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::{Request, Response};

pub struct ErrorLoggerFairing {}

impl Fairing for ErrorLoggerFairing {
    fn info(&self) -> Info {
        Info {
            name: "My Error logging fairing",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if response.status() == Status::InternalServerError {
            error!("Returned internal server error. Request: {}", request);
        }
    }
}
