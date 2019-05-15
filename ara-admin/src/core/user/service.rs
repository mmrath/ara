use ara_common::context::Context;
use ara_error::{AppError, HttpStatus};
use ara_model::core::{NewUser, User, UserRecord, WithAudit, AuditExt};
use ara_model::db::tx;
use failure::Fail;
use serde::Serialize;

pub fn create_user(
    context: &dyn Context,
    new_user: &NewUser<'_>,
) -> Result<UserRecord, UserServiceError> {
    tx(context.db(), |conn| {
        let maybe_user = User::find_by_username(conn, new_user.email)?;
        if maybe_user.is_some() {
            Err(UserServiceErrorKind::UserEmailAlreadyExists {
                email: new_user.email.to_owned(),
            })?
        }
        let user = User::insert(conn, WithAudit::new(context.username(), new_user))?;
        Ok(user)
    })
}

pub fn find_user_by_id(context: &dyn Context, id: i64) -> Result<UserRecord, UserServiceError> {
    tx(context.db(), |conn| {
        let user =
            User::find_by_id(conn, id)?.ok_or_else(|| UserServiceErrorKind::UserNotFound { id })?;
        Ok(user)
    })
}

pub fn update_user(
    context: &dyn Context,
    record: &User,
) -> Result<UserRecord, UserServiceError> {
    tx(context.db(), |conn| {
        let user = User::update(conn, record.with_audit(context.username()))?;
        Ok(user)
    })
}

type UserServiceError = AppError<UserServiceErrorKind>;

#[derive(Debug, Serialize, Fail, HttpStatus)]
pub enum UserServiceErrorKind {
    #[fail(display = "User already exists with same email")]
    #[http_status(400)]
    UserEmailAlreadyExists { email: String },

    // Send bad request even when not found
    #[fail(display = "User not found")]
    #[http_status(400)]
    UserNotFound { id: i64 },
}
