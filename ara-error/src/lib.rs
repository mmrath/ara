use failure::{Backtrace, Context, Fail};
use serde::Serialize;
use serde_json::Value;
use serde_type_name::type_name;
use std::fmt::{self, Display};

#[allow(unused_imports)]
#[macro_use]
extern crate ara_error_macros;

#[doc(hidden)]
pub use ara_error_macros::*;

#[derive(Debug)]
pub struct CustomError<T: Serialize + Fail + Display> {
    inner: Context<T>,
}

impl<T: Serialize + Fail + Display> Fail for CustomError<T> {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl<T: Serialize + Fail + Display> fmt::Display for CustomError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl<T: Serialize + Fail + Display + Copy> CustomError<T> {
    pub fn map_err<E: Into<failure::Error>>(error_kind: T) -> impl Fn(E) -> CustomError<T> {
        move |err| CustomError {
            inner: err.into().context(error_kind),
        }
    }

    pub fn map_with_context<E: Into<failure::Error>>(error_kind: T, err: E) -> CustomError<T> {
        CustomError {
            inner: err.into().context(error_kind),
        }
    }
}

pub fn err_mapper<E: Into<failure::Error>, T: Serialize + Fail + Display + Copy>(
    error_kind: T,
) -> impl Fn(E) -> CustomError<T> {
    move |err| CustomError {
        inner: err.into().context(error_kind),
    }
}

impl<T: Serialize + Fail + Display> CustomError<T> {
    pub fn kind(&self) -> &T {
        self.inner.get_context()
    }
}

impl<T: Serialize + Fail + Display> Serialize for CustomError<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("MyError", 2)?;
        let type_name = type_name(self.inner.get_context()).unwrap_or("Unknown error");

        //Remove Kind
        let error_name = if type_name.ends_with("Kind") {
            type_name.split_at(type_name.len() - 4).0
        } else {
            type_name
        };

        state.serialize_field("error", error_name)?;
        state.serialize_field("kind", self.inner.get_context())?;
        state.end()
    }
}

impl<T: Serialize + Fail> From<T> for CustomError<T> {
    fn from(kind: T) -> CustomError<T> {
        CustomError {
            inner: Context::new(kind),
        }
    }
}

impl<T: Serialize + Fail> From<Context<T>> for CustomError<T> {
    fn from(inner: Context<T>) -> CustomError<T> {
        CustomError { inner }
    }
}

pub trait HttpResponse: Send + Sync + 'static {
    fn body(&self) -> Value;
    fn status(&self) -> u16;
}

/*
mod rocket_impl {
    use rocket::http::{ContentType, Status};
    use rocket::request::Request;
    use rocket::response::{Responder, Response};
    use rocket::response::content;
    use std::io::Cursor;

    impl<'r> Responder<'r> for &super::HttpResponse {
        fn respond_to(self, _req: &Request<'_>) -> Result<Response<'r>, Status> {
            let body = self.body();
            let status = Status::from_code(self.status())
                .unwrap_or_else(||Status::InternalServerError);
            Response::build()
                .status(status)
                .sized_body(Cursor::new(Vec::new()))
                .header(ContentType::JSON)
                .ok()
        }
    }
}
*/
