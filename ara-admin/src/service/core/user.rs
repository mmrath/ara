use ara_common::context::Context;
use ara_common::error::{ServiceError, ServiceErrorKind};
use ara_model::core::{NewUser, User, UserRecord};
use ara_model::db::tx;
use ara_error::{ResultExt, AppError, HandledError};
use serde::Serialize;
use failure::Fail;

pub fn create_user(context: &dyn Context, new_user: &NewUser<'_>) -> Result<User, UserServiceError> {

    tx(context.db(), |conn| {

        let maybe_user = User::find_by_username(conn, new_user.email)?;

        if maybe_user.is_some() {
            Err(UserServiceErrorKind::UserEmailAlreadyExists{email: new_user.email.to_owned()})?
        }

        let user = User::insert(conn, &new_user.into())?;
        Ok(user)
    })
}

pub fn find_user_by_id(context: &dyn Context, id: i64) -> Result<UserRecord, ServiceError> {
    tx(context.db(), |conn| {
        let user = User::find_by_id(conn, id)?;
        Ok(user)
    })
}


type UserServiceError = AppError<UserServiceErrorKind>;

#[derive(Debug, Serialize, Fail)]
pub enum UserServiceErrorKind {

    #[fail(display = "User already exists with same email")]
    UserEmailAlreadyExists{email: String},
}


impl HandledError for UserServiceErrorKind {
    fn status(&self) -> u16{
        400
    }
}