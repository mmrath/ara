use crate::shared::config::AppConfig;
use crate::shared::Context;
use crate::shared::{argon2_hash, argon2_verify, sha512};
use failure::ResultExt;
use ara_error::ApiError;
use ara_model::core::{User, UserCredential};
use ara_model::db::{tx, Connection, TxError};
use serde::Serialize;

pub fn change_password(
    context: &dyn Context,
    current_password: &str,
    new_password: &str,
) -> Result<(), PasswordChangeError> {
    tx(context.db(), |conn| {
        change_password_internal(
            conn,
            context.user(),
            current_password,
            new_password,
            AppConfig::get(),
        )
    })
}

fn change_password_internal(
    conn: &Connection,
    user: &User,
    current_password: &str,
    new_password: &str,
    config: &AppConfig,
) -> Result<(), PasswordChangeError> {
    let user = User::find_by_id(conn, user.id).context(PasswordChangeErrorKind::Internal)?;

    if user.active {
        Err(PasswordChangeErrorKind::AccountNotActive)?;
    }

    let user_credential = UserCredential::find_by_id(conn, user.id)
        .context(PasswordChangeErrorKind::Internal)?
        .ok_or_else(|| PasswordChangeErrorKind::InvalidCurrentPassword)?;

    let hash = user_credential
        .password_hash
        .as_ref()
        .ok_or_else(|| PasswordChangeErrorKind::InvalidCurrentPassword)?
        .as_str();;

    let cur_password_sha512 = sha512(current_password.as_ref());
    let valid = argon2_verify(
        &cur_password_sha512,
        config.security.secret_key.as_ref(),
        &hash,
    )
    .context(PasswordChangeErrorKind::Internal)?;

    if !valid {
        User::increment_failed_login_count(conn, user.id)
            .context(PasswordChangeErrorKind::Internal)?;
        Err(PasswordChangeErrorKind::InvalidCurrentPassword)?;
    }

    let new_password_sha512 = sha512(new_password.as_bytes());
    let new_password_hash = argon2_hash(&new_password_sha512, config.security.secret_key.as_ref())?;

    User::update_password_hash(conn, user.id, &new_password_hash)
        .context(PasswordChangeErrorKind::Internal)?;

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Error, ApiError)]
pub enum PasswordChangeErrorKind {
    #[error(display = "Invalid password")]
    #[api_error(http(400))]
    InvalidCurrentPassword,

    #[error(display = "Account is currently locked")]
    #[api_error(http(400))]
    AccountLocked,

    #[error(display = "Account is currently locked")]
    #[api_error(http(400))]
    AccountNotActive,

    #[error(display = "Internal error")]
    #[api_error(map_from(TxError, Error))]
    Internal,
}
