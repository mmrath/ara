use failure::ResultExt;
use serde::Serialize;

use ara_error::ApiError;
use ara_model::core::{User, UserCredential};
use ara_model::db::{tx, TxError};

use crate::shared::{sha256_hex, PlainContext};

pub fn activate(context: &dyn PlainContext, token: &str) -> Result<(), ActivationError> {
    tx(context.db(), |conn| {
        let token_hash = sha256_hex(token.as_bytes());
        let (user, uc) = UserCredential::find_by_activation_key(conn, &token_hash)
            .context(ActivationErrorKind::Internal)?
            .ok_or_else(|| ActivationErrorKind::InvalidToken)?; //token or user does not exists

        if uc.activated {
            Err(ActivationErrorKind::AlreadyActivated)?;
        }
        User::activate(conn, user.id).context(ActivationErrorKind::Internal)?;
        //ut.delete(conn).context(ActivationErrorKind::Internal)?;
        Ok(())
    })
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Error, ApiError)]
pub enum ActivationErrorKind {
    #[error(display = "Invalid activation token")]
    #[api_error(http(400))]
    InvalidToken,
    #[error(display = "Account is currently locked")]
    #[api_error(http(400))]
    AccountLocked,
    #[error(display = "Account is already active")]
    #[api_error(http(400))]
    AlreadyActive,
    #[error(display = "Account is already activated")]
    #[api_error(http(400))]
    AlreadyActivated,
    #[error(display = "Internal error")]
    #[api_error(map_from(TxError, Error), http(500))]
    Internal,
}
