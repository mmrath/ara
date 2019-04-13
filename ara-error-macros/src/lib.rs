#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![recursion_limit = "1024"]

use synstructure::decl_derive;

mod api_error;
mod from_error;
mod http_response;

decl_derive!([ApiError, attributes(api_error)] => api_error::custom_error_derive);
decl_derive!([FromError, attributes(from_error)] => from_error::from_error_derive);
decl_derive!([HttpResponse, attributes(http)] => http_response::http_response_derive);
