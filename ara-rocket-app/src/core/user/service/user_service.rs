use crate::shared::{Context, ServiceError};
use ara_model::core::{NewUser, User, UserRecord};
use ara_model::db::tx;
use ara_error::unexpected_error;

pub fn create_user(context: &dyn Context, new_user: &NewUser<'_>) -> Result<UserRecord, ServiceError> {
    tx(context.db(), |conn| {
        let user = User::insert(conn, &new_user.into())?;
        Ok(user)
    })
}

pub fn find_by_id(context: &dyn Context, id: i64) -> Result<UserRecord, ServiceError> {
    tx(context.db(), |conn| {
        let user = User::find_by_id(conn, id)?
            .ok_or_else(||unexpected_error("User not found"))?;
        Ok(user)
    })
}
