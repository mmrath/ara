use failure::{Error, Fail};

use ara_common::config::AppConfig;
use ara_common::context::{Context, UnauthContext};
use ara_common::template;
use ara_common::utils::{argon2_hash, argon2_verify, new_uuid, sha256_hex, sha512};
use ara_error::{unexpected_error, AppError, HttpStatus};
use ara_model::db::{tx, Connection};
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use serde_json::json;
//use std::time::Duration;
use ara_model::core::{
    create_notification, Body, NewNotification, NewUserRecord, NotificationType, User,
    UserCredential, UserRecord, UserSignUp,
};
use log::{debug, info};

pub fn activate(context: &dyn UnauthContext, token: &str) -> Result<(), ActivationError> {
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
    let user =
        User::find_by_id(conn, user.id)?.ok_or_else(|| unexpected_error("User not found"))?;

    if user.active {
        Err(PasswordChangeErrorKind::AccountNotActive)?;
    }

    let user_credential = UserCredential::find_by_id(conn, user.id)?
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
    )?;

    if !valid {
        User::increment_failed_login_count(conn, user.id)?;
        Err(PasswordChangeErrorKind::InvalidCurrentPassword)?;
    }

    let new_password_sha512 = sha512(new_password.as_bytes());
    let new_password_hash = argon2_hash(&new_password_sha512, config.security.secret_key.as_ref())?;

    User::update_password_hash(conn, user.id, &new_password_hash)?;

    Ok(())
}

#[derive(Debug, Serialize, Fail, HttpStatus)]
pub enum PasswordChangeErrorKind {
    #[fail(display = "Invalid password")]
    #[http_status(400)]
    InvalidCurrentPassword,

    #[fail(display = "Account is currently locked")]
    #[http_status(400)]
    AccountLocked,

    #[fail(display = "Account is currently locked")]
    #[http_status(400)]
    AccountNotActive,
}

pub type PasswordChangeError = AppError<PasswordChangeErrorKind>;

pub fn password_reset_init(
    context: &dyn UnauthContext,
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

        let email_body = create_reset_init_email_body(&user, &uuid, &config.server.base_url)?;
        send_password_reset_email(conn, &user, email_body)?;
        Ok(())
    })
}

fn create_reset_init_email_body(
    user: &UserRecord,
    token: &str,
    base_url: &str,
) -> Result<String, Error> {
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
    context: &dyn UnauthContext,
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

#[derive(Debug, Serialize, Fail, HttpStatus)]
pub enum PasswordResetErrorKind {
    #[fail(display = "User does not exists")]
    #[http_status(400)]
    UserDoesNotExists,

    #[fail(display = "Invalid reset token")]
    #[http_status(400)]
    InvalidToken,
}

pub type PasswordResetError = AppError<PasswordResetErrorKind>;

pub fn sign_up(
    context: &dyn UnauthContext,
    user_ac: &UserSignUp,
) -> Result<UserRecord, SignUpError> {
    tx(context.db(), |conn| {
        debug!("User to register {:?}", user_ac);
        let config = AppConfig::get();
        let maybe_user = User::find_by_username(conn, &*user_ac.email)?;

        if maybe_user.is_some() {
            let user = maybe_user.unwrap();
            if !user.active {
                info!("Deleting existing unactivated account to signup new user");
                User::delete(conn, user.id)?
            } else {
                Err(SignUpErrorKind::UserEmailAlreadyExists)?;
            }
        }

        let password_sha512 = sha512(user_ac.password.as_bytes());
        let password_hash = argon2_hash(&password_sha512, config.security.secret_key.as_ref())?;

        let new_user = NewUserRecord {
            first_name: user_ac.first_name.as_ref(),
            last_name: user_ac.last_name.as_ref(),
            username: user_ac.email.as_ref(),
            email: user_ac.email.as_ref(),
        };

        let user = User::insert(conn, &new_user)?;

        let uuid = new_uuid();
        let uuid_hash = sha256_hex(&uuid.as_bytes());
        let token_expires_at = Utc::now()
            + Duration::seconds(i64::from(config.security.user_activation_token_expiry_mins) * 60);

        let credentials = UserCredential {
            id: user.id,
            password_hash: Some(password_hash),
            expires_at: None,
            invalid_attempts: 0,
            locked: false,
            activation_key: Some(uuid_hash),
            activation_key_expires_at: Some(token_expires_at),
            activated: false,
            reset_key: None,
            reset_key_expires_at: None,
            reset_at: None,
            updated_at: Utc::now(),
            version: 0,
        };
        credentials.create(conn)?;
        let email_body = create_email_body(&user, &uuid, &config.server.base_url)?;
        send_activation_email(conn, &user, email_body)?;
        Ok(user)
    })
}

fn create_email_body(user: &UserRecord, token: &str, base_url: &str) -> Result<String, Error> {
    use serde_json::value::Map;
    let mut data = Map::new();
    data.insert("base_url".to_owned(), json!(base_url));
    data.insert("user".to_owned(), json!(user));
    data.insert("token".to_owned(), json!(token));
    let email_body = template::render("email/user_activation", &data)?;
    Ok(email_body)
}

fn send_activation_email(
    conn: &Connection,
    user: &UserRecord,
    email_body: String,
) -> Result<(), Error> {
    let new_notification = NewNotification {
        notification_type: NotificationType::Email,
        from: Some("support@test.com".to_owned()),
        subject: "Account activation".to_string(),
        body: Body::Html(email_body),
        to: vec![user.email.clone().into()],
        cc: vec![],
        bcc: vec![],
    };
    create_notification(conn, &new_notification)?;
    Ok(())
}

#[derive(Debug, Serialize, Fail, HttpStatus)]
pub enum SignUpErrorKind {
    #[fail(display = "User already exists with same email")]
    #[http_status(400)]
    UserEmailAlreadyExists,
}

pub type SignUpError = AppError<SignUpErrorKind>;
