use chrono::Duration;
use chrono::Utc;
use failure::{Error, Fail};
use log::debug;
use serde::Serialize;
use serde_json::json;

use ara_error::{AppError, HttpStatus};
use ara_model::core::{create_notification, Body, NewNotification, NotificationType, UserCredential, UserRecord, NewUser, WithAudit};
use ara_model::core::{User, UserSignUp};
use ara_model::db::{tx, Connection};

use ara_common::config::AppConfig;
use ara_common::context::UnauthContext;
use ara_common::template;
use ara_common::utils::{argon2_hash, new_uuid, sha256_hex, sha512};
use log::info;

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

        let new_user = NewUser {
            first_name: user_ac.first_name.as_ref(),
            last_name: user_ac.last_name.as_ref(),
            email: user_ac.email.as_ref(),
            phone_number: None,
        };

        let user = User::insert(conn, WithAudit::new("SIGNUP", &new_user))?;

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
