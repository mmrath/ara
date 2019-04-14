use crate::shared::db::DB;
use crate::shared::{jwt};
use crate::shared::{internal_server_error, HttpError, HttpResultExt, JsonResult};
use rocket::Route;
use rocket_contrib::json::{Json, JsonValue};
use ara_model::core::{NewUser, User, UserLogin, UserSignUp};
use ara_service::core::user;


#[post("/", format = "json", data = "<new_user>")]
fn create(new_user: Json<NewUser<'_>>, context: DB) -> JsonResult<User> {
    user::create_user(&context, &new_user.into_inner()).into_json()
}

#[get("/<id>", format = "json")]
fn get(id: i64, context: DB) -> Result<Json<User>, HttpError> {
    user::find_by_id(&context, id).map(User::from).into_json()
}
