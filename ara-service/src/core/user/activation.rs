use failure::{ResultExt,Fail};
use serde::Serialize;

use ara_error::{ApiError, BoxedError};
use ara_model::core::{User, UserCredential};
use ara_model::db::{tx, TxError};

use crate::shared::{sha256_hex, PlainContext};

pub fn activate(context: &dyn PlainContext, token: &str) -> Result<(), ActivationError> {
    tx(context.db(), |conn| {
        let token_hash = sha256_hex(token.as_bytes());
        let (user, uc) = UserCredential::find_by_activation_key(conn, &token_hash)?
            .ok_or_else(|| ActivationErrorKind::InvalidToken)?; //token or user does not exists

        if uc.activated {
            Err(ActivationErrorKind::AlreadyActivated)?;
        }
        User::activate(conn, user.id)?;
        //ut.delete(conn).context(ActivationErrorKind::Internal)?;
        Ok(())
    })
}

#[derive(Debug, Serialize, Fail, ApiError)]
pub enum ActivationErrorKind {
    #[fail(display = "Invalid activation token")]
    #[api_error(http(400))]
    InvalidToken,
    #[fail(display = "Account is currently locked")]
    #[api_error(http(400))]
    AccountLocked,
    #[fail(display = "Account is already active")]
    #[api_error(http(400))]
    AlreadyActive,
    #[fail(display = "Account is already activated")]
    #[api_error(http(400))]
    AlreadyActivated,

    #[fail(display = "{}", _0)]
    #[api_error(map_from(Error), http(500))]
    Internal(BoxedError),
}
