use crate::shared::db::DB;
use crate::shared::error::HttpError;
use ara_model::core::{NewRole, Role, RoleUpdate};
use ara_service::core::role;
use tide::body::Json;
use tide::head;

pub async fn find(id: head::Path<i32>, conn: DB) -> Result<Json<Role>, HttpError> {
    let r = role::find(&conn, id.0)?;
    Ok(Json(r))
}

pub async fn create(new_role: Json<NewRole<'_>>, conn: DB) -> Result<Json<Role>, HttpError> {
    let r = role::create(&conn, &new_role.0)?;
    Ok(Json(r))
}

pub async fn update(role_update: Json<RoleUpdate>, conn: DB) -> Result<Json<Role>, HttpError> {
    let r = role::update(&conn, &role_update.0)?;
    Ok(Json(r))
}

pub async fn delete(id: head::Path<i32>, conn: DB) -> Result<(), HttpError> {
    role::delete(&conn, id.0)?;
    Ok(())
}
