use crate::shared::{Context, ServiceError};
use ara_model::core::{NewUser, User, UserRecord};
use ara_model::db::tx;

pub fn create_user(context: &dyn Context, new_user: &NewUser<'_>) -> Result<User, ServiceError> {
    tx(context.db(), |conn| {
        let user = User::insert(conn, &new_user.into())?;
        Ok(user)
    })
}

pub fn find_by_id(context: &dyn Context, id: i64) -> Result<UserRecord, ServiceError> {
    tx(context.db(), |conn| {
        let user = User::find_by_id(conn, id)?;
        Ok(user)
    })
}
