use ara_common::context::Context;
use ara_error::{AppError, HttpStatus};
use ara_model::core::{User, UserRecord, WithAudit};
use ara_model::db::tx;
use failure::Fail;
use serde::Serialize;

pub fn update_profile(
    context: &dyn Context,
    updated_user: &User,
) -> Result<UserRecord, UserServiceError> {
    tx(context.db(), |conn| {
        if updated_user.id != context.user().id {
            Err(UserServiceErrorKind::NotAuthorizedToModifyProfile)?
        }
        let user = User::update(conn, WithAudit::new(context.user().username.as_str(), updated_user))?;
        Ok(user)
    })
}

pub fn get_profile(context: &dyn Context) -> Result<User, UserServiceError> {
    Ok(context.user().clone())
}



type UserServiceError = AppError<UserServiceErrorKind>;

#[derive(Debug, Serialize, Fail, HttpStatus)]
pub enum UserServiceErrorKind {
    #[fail(display = "User already exists with same email")]
    #[http_status(400)]
    UserEmailAlreadyExists { email: String },

    #[fail(display = "User can only modify their own profile")]
    #[http_status(401)]
    NotAuthorizedToModifyProfile,


    // Send bad request even when not found
    #[fail(display = "User not found")]
    #[http_status(400)]
    UserNotFound { id: i64 },
}
