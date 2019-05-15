use crate::shared::config::AppConfig;
use crate::shared::PlainContext;
use crate::shared::{argon2_verify, sha512};
use ara_error::{ApiError, BoxedError};
use ara_model::core::{User, UserCredential};
use ara_model::db::{tx, Connection};
use chrono::Utc;
use failure::Fail;
use serde::Serialize;

pub fn login(
    context: &dyn PlainContext,
    username: &str,
    password: &str,
) -> Result<User, LoginError> {
    tx(context.db(), |conn| {
        let config = AppConfig::get();
        login_internal(
            conn,
            username,
            password,
            config.security.secret_key.as_bytes(),
        )
    })
}
fn login_internal(
    conn: &Connection,
    username: &str,
    password: &str,
    secret_key: &[u8],
) -> Result<User, LoginError> {
    let user = User::find_by_username(conn, username)?
        .ok_or_else(|| LoginErrorKind::InvalidUsernameOrPassword)?;

    let user_credential = UserCredential::find_by_id(conn, user.id)?
        .ok_or_else(|| LoginErrorKind::InvalidUsernameOrPassword)?;

    let hash = user_credential
        .password_hash
        .as_ref()
        .ok_or_else(|| LoginErrorKind::InvalidUsernameOrPassword)?
        .as_str();

    let password_sha512 = sha512(password.as_ref());

    let valid = argon2_verify(&password_sha512, secret_key, &hash)?;

    if !valid {
        User::increment_failed_login_count(conn, user.id)?;
        Err(LoginErrorKind::InvalidUsernameOrPassword)?;
    } else if !user.active {
        Err(LoginErrorKind::AccountNotActivated)?;
    } else if user_credential
        .expires_at
        .map(|exp| exp > Utc::now())
        .unwrap_or(false)
    {
        Err(LoginErrorKind::PasswordExpired)?;
    } else if user_credential.invalid_attempts > 0 {
        User::reset_failed_login_count(conn, user.id)?;
    }
    Ok(User::from(user))
}

#[derive(Debug, Serialize, Fail, ApiError)]
pub enum LoginErrorKind {
    #[fail(display = "Invalid username or password")]
    #[api_error(http(401))]
    InvalidUsernameOrPassword,

    #[fail(display = "Account is currently locked")]
    #[api_error(http(401))]
    AccountLocked,

    #[fail(display = "Account is not activated")]
    #[api_error(http(401))]
    AccountNotActivated,

    #[fail(display = "Password expired")]
    #[api_error(http(401))]
    PasswordExpired,

    #[fail(display = "Internal error")]
    #[api_error(map_from(Error), http(500))]
    Internal(BoxedError),
}
