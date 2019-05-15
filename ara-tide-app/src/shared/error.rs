use std::fmt::{self, Display};

use http::StatusCode;
use http_service::Body;
use log::{error, info};
use serde::Serialize;
use serde_json::{json, Value};
use tide::response::{IntoResponse, Response};

#[derive(Debug)]
pub struct HttpError {
    body: Value,
    status: StatusCode,
}

impl HttpError {
    #[allow(dead_code)]
    /// Set the data of the `Response` to `data`.
    pub(crate) fn new<T: Serialize>(status: StatusCode, data: &T) -> HttpError {
        Self {
            body: json!(data),
            status,
        }
    }

    #[allow(dead_code)]
    /// Set the data of the `Response` to `data`.
    pub(crate) fn body<T: Serialize>(mut self, data: T) -> HttpError {
        self.body = json!(&data);
        self
    }

    #[allow(dead_code)]
    /// Convenience method to set `self.data` to `{"message": message}`.
    pub(crate) fn message(mut self, message: &str) -> HttpError {
        self.body = json!({ "message": message });
        self
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.body, self.status)
    }
}

impl<T> From<T> for HttpError
where
    T: ara_error::HttpResponse,
{
    fn from(e: T) -> Self {
        let status = StatusCode::from_u16(e.status()).unwrap_or_else(|e| {
            error!("Error converting status code {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        });

        HttpError {
            body: e.body(),
            status,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let body_data = serde_json::to_vec(&self.body);
        info!("Converting ApiError to response {:?}", self);
        body_data
            .map(|data| {
                http::Response::builder()
                    .status(self.status)
                    .header("Content-Type", "application/json")
                    .body(Body::from(data))
                    .unwrap()
            })
            .unwrap_or_else(|_e| {
                http::Response::builder()
                    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(vec![]))
                    .unwrap()
            })
    }
}

#[allow(dead_code)]
pub(crate) fn ok() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::OK,
    }
}

#[allow(dead_code)]
pub(crate) fn created() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::CREATED,
    }
}

#[allow(dead_code)]
pub(crate) fn accepted() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::ACCEPTED,
    }
}

#[allow(dead_code)]
pub(crate) fn no_content() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::NO_CONTENT,
    }
}

#[allow(dead_code)]
pub(crate) fn bad_request() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::BAD_REQUEST,
    }
}

#[allow(dead_code)]
pub(crate) fn unauthorized() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::UNAUTHORIZED,
    }
}

#[allow(dead_code)]
pub(crate) fn forbidden() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::FORBIDDEN,
    }
}

#[allow(dead_code)]
pub(crate) fn not_found() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::NOT_FOUND,
    }
}

#[allow(dead_code)]
pub(crate) fn method_not_allowed() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::METHOD_NOT_ALLOWED,
    }
}

#[allow(dead_code)]
pub(crate) fn conflict() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::CONFLICT,
    }
}

#[allow(dead_code)]
pub(crate) fn unprocessable_entity() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::UNPROCESSABLE_ENTITY,
    }
}

#[allow(dead_code)]
pub(crate) fn internal_server_error() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[allow(dead_code)]
pub(crate) fn service_unavailable() -> HttpError {
    HttpError {
        body: json!(null),
        status: StatusCode::SERVICE_UNAVAILABLE,
    }
}
