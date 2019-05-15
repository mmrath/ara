use failure::Fail;
use serde::Serialize;

use ara_error::{AppError, HttpStatus};
use ara_model::core::{User, UserCredential};
use ara_model::db::tx;

use ara_common::context::UnauthContext;
use ara_common::utils::sha256_hex;

pub fn activate(context: &dyn UnauthContext, token: &str) -> Result<(), ActivationError> {
    tx(context.db(), |conn| {
        let token_hash = sha256_hex(token.as_bytes());
        let (user, uc) = UserCredential::find_by_activation_key(conn, &token_hash)?
            .ok_or_else(|| ActivationErrorKind::InvalidToken)?; //token or user does not exists

        if uc.activated {
            Err(ActivationErrorKind::AlreadyActivated)?;
        }
        User::activate(conn, user.id)?;
        Ok(())
    })
}

#[derive(Debug, Serialize, Fail, HttpStatus)]
pub enum ActivationErrorKind {
    #[fail(display = "Invalid activation token")]
    #[http_status(400)]
    InvalidToken,
    #[fail(display = "Account is currently locked")]
    #[http_status(400)]
    AccountLocked,
    #[fail(display = "Account is already active")]
    #[http_status(400)]
    AlreadyActive,
    #[fail(display = "Account is already activated")]
    #[http_status(400)]
    AlreadyActivated,
}
pub type ActivationError = AppError<ActivationErrorKind>;
