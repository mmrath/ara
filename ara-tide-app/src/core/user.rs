use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tide::body::Json;
use tide::head::Path;

use ara_model::core::{User, UserLogin, UserSignUp};
use ara_service::core::user;

use crate::shared::db::DB;
use crate::shared::error::internal_server_error;
use crate::shared::error::HttpError;
use crate::shared::jwt;

pub async fn sign_up(sign_up: Json<UserSignUp>, conn: DB) -> Result<Json<User>, HttpError> {
    user::sign_up(&conn, &sign_up.0)
        .map(Json)
        .map_err(From::from)
}

pub async fn login(login: Json<UserLogin>, conn: DB) -> Result<Json<Value>, HttpError> {
    let user: User = user::login(&conn, &login.username, &login.password)?;
    let jwt_token = jwt::new_token(&user).map_err(|_err| internal_server_error())?;
    Ok(Json(json!({ "token": jwt_token })))
}

pub async fn activate(Path(token): Path<String>, conn: DB) -> Result<(), HttpError> {
    user::activate(&conn, &token)?;
    Ok(())
}

pub async fn password_reset_init(email: Json<String>, conn: DB) -> Result<(), HttpError> {
    user::password_reset_init(&conn, &email)?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PasswordWithResetToken {
    pub token: String,
    pub password: String,
}

pub async fn password_reset_finish(
    password_reset: Json<PasswordWithResetToken>,
    conn: DB,
) -> Result<(), HttpError> {
    user::password_reset_finish(&conn, &*password_reset.token, &*password_reset.password)?;
    Ok(())
}
