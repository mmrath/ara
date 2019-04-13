use crate::shared::config::AppConfig;
use crate::shared::{argon2_hash, new_uuid, sha256_hex, sha512, template, PlainContext};
use chrono::{Duration, Utc};
use failure::{Error, ResultExt};
use ara_error::ApiError;
use ara_model::core::{create_notification, Body, NewNotification, NotificationType, User, UserRecord, UserCredential};
use ara_model::db::{tx, Connection, TxError};
use serde::Serialize;

pub fn password_reset_init(
    context: &dyn PlainContext,
    email: &str,
) -> Result<(), PasswordResetError> {
    tx(context.db(), |conn| {
        let config = AppConfig::get();
        let user = User::find_by_username(conn, email)
            .context(PasswordResetErrorKind::Internal)?
            .ok_or_else(|| PasswordResetErrorKind::UserDoesNotExists)?;

        let uuid = new_uuid();
        let uuid_hash = sha256_hex(&uuid.as_bytes());
        let expires_at = Utc::now()
            + Duration::seconds(
                i64::from(config.security.user_password_reset_token_expiry_mins) * 60,
            );

        UserCredential::update_reset_key(conn, user.id, &uuid_hash, expires_at)
            .context(PasswordResetErrorKind::Internal)?;

        let email_body = create_email_body(&user, &uuid, &config.server.base_url)
            .context(PasswordResetErrorKind::Internal)?;
        send_password_reset_email(conn, &user, email_body)
            .context(PasswordResetErrorKind::Internal)?;
        Ok(())
    })
}

fn create_email_body(user: &UserRecord, token: &str, base_url: &str) -> Result<String, Error> {
    use serde_json::json;
    use serde_json::value::Map;

    let mut data = Map::new();

    data.insert("base_url".to_owned(), json!(base_url));
    data.insert("user".to_owned(), json!(user));
    data.insert("token".to_owned(), json!(token));

    let email_body = template::render("email/password_reset", &data)
        .context(PasswordResetErrorKind::Internal)?;
    Ok(email_body)
}

pub fn password_reset_finish(
    context: &dyn PlainContext,
    token: &str,
    new_password: &str,
) -> Result<(), PasswordResetError> {
    tx(context.db(), |conn| {
        let token_hash = sha256_hex(token.as_bytes());
        let (user, _uc) = UserCredential::find_by_reset_key(conn, &token_hash)
            .context(PasswordResetErrorKind::Internal)?
            .ok_or_else(|| PasswordResetErrorKind::InvalidToken)?; //User does not exists

        set_user_password(
            user.id,
            new_password,
            AppConfig::get().security.secret_key.as_bytes(),
            conn,
        )
        .context(PasswordResetErrorKind::Internal)?;
        Ok(())
    })
}

fn set_user_password(
    user_id: i64,
    new_password: &str,
    enc_key: &[u8],
    conn: &Connection,
) -> Result<(), Error> {
    let password_sha512 = sha512(new_password.as_bytes());
    let password_hash = argon2_hash(&password_sha512, enc_key)?;

    User::set_password(conn, user_id, password_hash.as_ref())?;
    Ok(())
}

fn send_password_reset_email(
    conn: &Connection,
    user: &UserRecord,
    email_body: String,
) -> Result<(), Error> {
    let new_notification = NewNotification {
        notification_type: NotificationType::Email,
        from: Some("support@test.com".to_owned()),
        subject: "Password Reset".to_string(),
        body: Body::Html(email_body),
        to: vec![user.email.clone().into()],
        cc: vec![],
        bcc: vec![],
    };

    create_notification(conn, &new_notification).context(PasswordResetErrorKind::Internal)?;
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Error, ApiError)]
pub enum PasswordResetErrorKind {
    #[error(display = "User does not exists")]
    UserDoesNotExists,

    #[error(display = "Invalid reset token")]
    InvalidToken,

    #[error(display = "Internal error")]
    #[api_error(map_from(TxError))]
    Internal,
}