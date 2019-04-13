use crate::shared::SessionStore;
use crate::shared::{internal_server_error, HttpError, HttpResultExt, JsonResult};
use crate::shared::{jwt, HashMapSessionStore};
use crate::shared::{AuthContext, UnauthContext};
use rocket::{Route, State};
use rocket_contrib::json::{Json, JsonValue};
use ara_model::core::{NewUser, User, UserLogin, UserSignUp};
use ara_service::core::user;
use uuid::Uuid;

use failure::format_err;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub fn routes() -> Vec<Route> {
    routes![
        self::create,
        self::get,
        self::sign_up,
        self::login,
        self::activate,
        self::logout,
        self::password_reset_init,
        self::password_reset_finish,
        self::change_password,
    ]
}

#[post("/", format = "json", data = "<new_user>")]
fn create(new_user: Json<NewUser<'_>>, context: AuthContext) -> JsonResult<User> {
    user::create_user(&context, &new_user.into_inner()).into_json()
}

#[get("/<id>", format = "json")]
fn get(id: i64, context: AuthContext) -> Result<Json<User>, HttpError> {
    user::find_by_id(&context, id).map(User::from).into_json()
}

#[post("/signup", format = "json", data = "<signup>")]
fn sign_up(signup: Json<UserSignUp>, context: UnauthContext) -> JsonResult<User> {
    user::sign_up(&context, &signup.0).into_json()
}

#[post("/login", format = "json", data = "<login>")]
fn login(
    login: Json<UserLogin>,
    context: UnauthContext,
    session_store: State<Arc<Mutex<HashMapSessionStore<String, User>>>>,
) -> Result<JsonValue, HttpError> {
    let user: User = user::login(&context, &login.username, &login.password)?;
    let jwt_token = jwt::new_token(&user).map_err(internal_server_error)?;
    let session_id = Uuid::new_v4().to_string();
    session_store
        .inner()
        .clone()
        .lock()
        .map_err(|e| format_err!("Unable to acquire lock {:?}", e))
        .map_err(internal_server_error)?
        .insert(session_id, user);
    Ok(json!({ "token": jwt_token }))
}

#[post("/logout")]
fn logout() -> Result<(), HttpError> {
    //cookies.remove("jwt");
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
#[get("/activate/<token>")]
fn activate(token: String, context: UnauthContext) -> JsonResult<()> {
    user::activate(&context, &token).into_json()
}

#[allow(clippy::needless_pass_by_value)]
#[post("/password-reset/init", format = "json", data = "<email>")]
fn password_reset_init(email: String, context: UnauthContext) -> JsonResult<()> {
    user::password_reset_init(&context, &email).into_json()
}

#[allow(clippy::needless_pass_by_value)]
#[post("/password-reset/finish", format = "json", data = "<password_reset>")]
fn password_reset_finish(
    password_reset: Json<PasswordWithResetToken>,
    context: UnauthContext,
) -> JsonResult<()> {
    user::password_reset_finish(&context, &*password_reset.token, &*password_reset.password)
        .into_json()
}

#[post("/password-change", format = "json", data = "<password_change>")]
fn change_password(password_change: Json<PasswordChange>, context: AuthContext) -> JsonResult<()> {
    user::change_password(
        &context,
        &*password_change.current_password,
        &*password_change.new_password,
    )
    .into_json()
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChange {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PasswordWithResetToken {
    pub token: String,
    pub password: String,
}
