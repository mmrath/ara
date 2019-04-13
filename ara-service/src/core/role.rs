use crate::shared::ServiceError;
use ara_model::core::{NewRole, Role, RoleUpdate};
use ara_model::db::{tx, Connection};

pub fn find(conn: &Connection, role_id: i32) -> Result<Role, ServiceError> {
    let r = Role::find(conn, role_id)?;
    Ok(r)
}

pub fn create(conn: &Connection, new_role: &NewRole<'_>) -> Result<Role, ServiceError> {
    let role = tx(conn, |conn| Role::create(conn, &new_role))?;
    Ok(role)
}

pub fn update(conn: &Connection, role_update: &RoleUpdate) -> Result<Role, ServiceError> {
    let role = tx(conn, |conn| Role::update(conn, &role_update))?;
    Ok(role)
}

pub fn delete(conn: &Connection, role_id: i32) -> Result<(), ServiceError> {
    tx(conn, |conn| Role::delete(conn, role_id))?;
    Ok(())
}
