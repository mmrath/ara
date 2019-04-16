use failure::{Backtrace, Context, Fail};
use serde::Serialize;
use serde_json::Value;
use serde_type_name::type_name;
use std::fmt::{self, Display};
use std::error::Error;
use log::{info,error};


#[allow(unused_imports)]
#[macro_use]
extern crate ara_error_macros;

#[doc(hidden)]
pub use ara_error_macros::*;

pub trait HttpResponse: Send + Sync + 'static {
    fn body(&self) -> Value;
    fn status(&self) -> u16;
}


pub trait ApiErrorKind: Fail + Sized {
    type Error: Fail + From<Context<Self>>;
}

/// Extension trait which adds the family of `.chain_err` methods to `Result` objects.
pub trait ResultExt: Sized {
    type Success;
    type Error: Fail;

    /// Replace the error in a Result with a new error built from `map`'s `ErrorKind` output.
    ///
    /// The original error is stored as the `cause`/`source` of the new one.
    fn chain_err<ErrorKindT: ApiErrorKind>(
        self,
        map: impl FnOnce() -> ErrorKindT,
    ) -> Result<Self::Success, ErrorKindT::Error> {
        self.chain_inspect_err(|_| map())
    }

    /// Like `chain_err`, but the callback is given an opportunity to inspect the original error.
    fn chain_inspect_err<ErrorKindT: ApiErrorKind>(
        self,
        map: impl FnOnce(&mut Self::Error) -> ErrorKindT,
    ) -> Result<Self::Success, ErrorKindT::Error>;
}

impl<SuccessT, ErrorT: Fail> ResultExt for Result<SuccessT, ErrorT> {
    type Success = SuccessT;
    type Error = ErrorT;

    fn chain_inspect_err<ErrorKindT: ApiErrorKind>(
        self,
        chain: impl FnOnce(&mut ErrorT) -> ErrorKindT,
    ) -> Result<Self::Success, ErrorKindT::Error> {
        self.map_err(|mut initial_error| {
            let kind = chain(&mut initial_error);
            initial_error.context(kind).into()
        })
    }
}


// This is to skip serialization of inner value
#[derive(Debug, Fail, Serialize)]
#[fail(display = "{}", inner)]
pub struct BoxedError {
    #[serde(skip_serializing)]
    inner: failure::Error
}

impl From<failure::Error> for BoxedError{
    fn from(err: failure::Error) -> Self {
        Self{
            inner: err
        }
    }
}


/// Returns early with an error built from an error kind.
///
/// Examples
/// ---
///
/// ```rust
/// // With an ErrorKind.
/// bail!(ErrorKind::CorruptFile(format!("the file at {:?} is corrupt", file_path)))
///
/// // With an ErrorKind and format string (equivalent ot the above.)
/// bail!(ErrorKind::CorruptFile, "the file at {:?} is corrupt", file_path)
/// ```
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };
    ($e:expr,) => {
        return Err($e.into());
    };
    ($kind:expr, $fmt:expr) => {
        return Err($kind(($fmt).to_owned()).into());
    };
    ($kind:expr, $fmt:expr, $($arg:tt)+) => {
        return Err($kind(format!($fmt, $($arg)+)).into());
    };
}


/// Returns early with an error built from an error kind if a given condition is false.
///
/// Examples
/// ---
///
/// ```rust
/// // With an ErrorKind.
/// ensure!(
///     file_path != corrupt_file_path,
///     ErrorKind::CorruptFile(format!("the file at {:?} is corrupt", file_path))
/// );
///
/// // With an ErrorKind and format string (equivalent ot the above.)
/// ensure!(
///     file_path != corrupt_file_path,
///     ErrorKind::CorruptFile,
///     "the file at {:?} is corrupt",
///     file_path,
/// );
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr, $fmt:expr, $($arg:tt)+) => {
        if !($cond) {
            $crate::bail!($e, $fmt, $($arg)+);
        }
    };
    ($cond:expr, $e:expr, $fmt:expr) => {
        if !($cond) {
            $crate::bail!($e, $fmt);
        }
    };
    ($cond:expr, $e:expr,) => {
        if !($cond) {
            $crate::bail!($e);
        }
    };
    ($cond:expr, $e:expr) => {
        if !($cond) {
            $crate::bail!($e);
        }
    };
}









































impl<T: Serialize + Fail + Display> Serialize for AraError<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("AraError", 2)?;
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


pub trait HttpStatus: Send + Sync + 'static {
    fn status(&self) -> u16;
}

pub trait ErrorKindFrom<T>: Send + Sync + 'static {
    fn as_kind(e: &T) -> Self;
}

impl<T> HttpResponse for AraError<T> where T: HttpStatus + Serialize + Fail {
    fn body(&self) -> Value {
        let val = serde_json::to_value(self).unwrap_or_else(|e| {
            error!("Error converting to json Value {:?}. Value will be null", e);
            Value::Null
        });
        info!("Returned value {}", val.to_string());
        val
    }

    fn status(&self) -> u16 {
        self.kind().status()
    }
}

#[derive(Debug)]
pub struct AraError<ErrorKindT: Fail> {
    inner: Context<ErrorKindT>,
}

impl<ErrorKindT: Fail> AraError<ErrorKindT> {
    pub fn kind(&self) -> &ErrorKindT {
        self.inner.get_context()
    }
}



impl<T: Serialize + Fail + Display + Copy> AraError<T> {
    pub fn map_err<E: Into<failure::Error>>(error_kind: T) -> impl Fn(E) -> Self {
        move |err| Self {
            inner: err.into().context(error_kind),
        }
    }

    pub fn map_with_context<E: Into<failure::Error>>(error_kind: T, err: E) -> Self {
        Self {
            inner: err.into().context(error_kind),
        }
    }

}

impl<ErrorKindT: Fail> Fail for AraError<ErrorKindT> {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl<ErrorKindT: Fail> fmt::Display for AraError<ErrorKindT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<ErrorKindT: Fail> From<ErrorKindT> for AraError<ErrorKindT> {
    fn from(kind: ErrorKindT) -> Self {
        Self::from(Context::new(kind))
    }
}



impl<ErrorKindT: Fail> From<Context<ErrorKindT>> for AraError<ErrorKindT> {
    fn from(inner: Context<ErrorKindT>) -> Self {
        Self { inner }
    }
}












#[derive(Debug)]
pub enum AppError<K:Fail>{
    Handled(Context<K>),
    Unhandled(BoxedError),
}

impl<K: Fail> Fail for AppError<K> {
    fn cause(&self) -> Option<&Fail> {
        match self {
            AppError::Handled(inner) => {inner.cause()},
            AppError::Unhandled(box_error) => {Some(box_error.inner.as_fail())}
        }
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        match self {
            AppError::Handled(inner) => {inner.backtrace()},
            AppError::Unhandled(box_error) => {Some(box_error.inner.backtrace())}
        }
    }
}

impl<K: Fail> fmt::Display for AppError<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Handled(inner) => {inner.fmt(f)},
            AppError::Unhandled(box_error) => {box_error.inner.fmt(f)}
        }
    }
}


impl<K: Fail+HandledError> From<K> for AppError<K> {
    fn from(kind: K) -> Self {
        Self::from(Context::new(kind))
    }
}

impl<K: Fail + HandledError> From<Context<K>> for AppError<K> {
    fn from(inner: Context<K>) -> Self {
        AppError::Handled(inner)
    }
}

impl<K: Fail+HandledError> From<failure::Error> for AppError<K> {
    fn from(inner: failure::Error) -> Self {
        AppError::Unhandled(BoxedError{inner})
    }
}

impl<K> ApiErrorKind for K where K:Serialize + Fail + HandledError{
    type Error= AppError<Self>;
}

pub trait HandledError: Serialize {
    fn status(&self) -> u16;
}

impl<T: Serialize + Fail + HandledError> Serialize for AppError<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("AppError", 2)?;

        let type_name = match self {
            AppError::Handled(inner) => {
                type_name(inner.get_context()).unwrap_or("InternalError")
            },
            AppError::Unhandled(_) => {"InternalError"}
        };


        //Remove Kind
        let error_name = if type_name.ends_with("Kind") {
            type_name.split_at(type_name.len() - 4).0
        } else {
            type_name
        };

        state.serialize_field("error", error_name)?;

        match self {
            AppError::Handled(inner) => {
                state.serialize_field("kind", inner.get_context())?;
            },
            AppError::Unhandled(_) => {
                state.serialize_field("kind", "Unknown")?;
            }
        };

        state.end()
    }
}


impl<T> HttpResponse for AppError<T> where T: Serialize + Fail + HandledError {
    fn body(&self) -> Value {
        let val = serde_json::to_value(self).unwrap_or_else(|e| {
            error!("Error converting to json Value {:?}. Value will be null", e);
            Value::Null
        });
        info!("Returned value {}", val.to_string());
        val
    }

    fn status(&self) -> u16 {
        match self {
            AppError::Handled(inner) => {
                inner.get_context().status()
            },
            AppError::Unhandled(_) => {
                500
            }
        }
    }
}
