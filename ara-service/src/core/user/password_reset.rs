use crate::shared::config::AppConfig;
use crate::shared::{argon2_hash, new_uuid, sha256_hex, sha512, template, PlainContext};
use ara_error::{ApiError, BoxedError};
use ara_model::core::{
    create_notification, Body, NewNotification, NotificationType, User, UserCredential, UserRecord,
};
use ara_model::db::{tx, Connection, TxError};
use chrono::{Duration, Utc};
use failure::{Error, ResultExt, Fail};
use serde::Serialize;

pub fn password_reset_init(
    context: &dyn PlainContext,
    email: &str,
) -> Result<(), PasswordResetError> {
    tx(context.db(), |conn| {
        let config = AppConfig::get();
        let user = User::find_by_username(conn, email)?
            .ok_or_else(|| PasswordResetErrorKind::UserDoesNotExists)?;

        let uuid = new_uuid();
        let uuid_hash = sha256_hex(&uuid.as_bytes());
        let expires_at = Utc::now()
            + Duration::seconds(
                i64::from(config.security.user_password_reset_token_expiry_mins) * 60,
            );

        UserCredential::update_reset_key(conn, user.id, &uuid_hash, expires_at)?;

        let email_body = create_email_body(&user, &uuid, &config.server.base_url)?;
        send_password_reset_email(conn, &user, email_body)?;
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

    let email_body = template::render("email/password_reset", &data)?;
    Ok(email_body)
}

pub fn password_reset_finish(
    context: &dyn PlainContext,
    token: &str,
    new_password: &str,
) -> Result<(), PasswordResetError> {
    tx(context.db(), |conn| {
        let token_hash = sha256_hex(token.as_bytes());
        let (user, _uc) = UserCredential::find_by_reset_key(conn, &token_hash)?
            .ok_or_else(|| PasswordResetErrorKind::InvalidToken)?; //User does not exists

        set_user_password(
            user.id,
            new_password,
            AppConfig::get().security.secret_key.as_bytes(),
            conn,
        )?;
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

    create_notification(conn, &new_notification)?;
    Ok(())
}

#[derive(Debug, Serialize, Fail, ApiError)]
pub enum PasswordResetErrorKind {
    #[fail(display = "User does not exists")]
    UserDoesNotExists,

    #[fail(display = "Invalid reset token")]
    InvalidToken,

    #[fail(display = "{}", _0)]
    #[api_error(map_from(Error), http(500))]
    Internal(BoxedError),
}
