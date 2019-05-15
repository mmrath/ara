use ara_common::config::AppConfig;
use ara_common::context::UnauthContext;
use ara_common::utils::{argon2_verify, sha512};
use ara_error::{AppError, HttpStatus};
use ara_model::core::{User, UserCredential};
use ara_model::db::{tx, Connection};
use chrono::Utc;
use failure::Fail;
use serde::Serialize;

pub fn login(
    context: &dyn UnauthContext,
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

#[derive(Debug, Serialize, Fail, HttpStatus)]
pub enum LoginErrorKind {
    #[fail(display = "Invalid username or password")]
    #[http_status(401)]
    InvalidUsernameOrPassword,

    #[fail(display = "Account is currently locked")]
    #[http_status(401)]
    AccountLocked,

    #[fail(display = "Account is not activated")]
    #[http_status(401)]
    AccountNotActivated,

    #[fail(display = "Password expired")]
    #[http_status(401)]
    PasswordExpired,
}

pub type LoginError = AppError<LoginErrorKind>;
