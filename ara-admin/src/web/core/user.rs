use crate::service::core::user;
use crate::shared::AuthContext;
use crate::shared::{HttpError, HttpResultExt, JsonResult};
use ara_model::core::{NewUser, User};
use rocket::Route;
use rocket_contrib::json::Json;

pub fn routes() -> Vec<Route> {
    routes![self::create, self::get, self::update,]
}

#[post("/", format = "json", data = "<new_user>")]
fn create(new_user: Json<NewUser<'_>>, context: AuthContext) -> JsonResult<User> {
    user::create_user(&context, &new_user.into_inner()).into_json()
}

#[get("/<id>", format = "json")]
fn get(id: i64, context: AuthContext) -> Result<Json<User>, HttpError> {
    user::find_user_by_id(&context, id)
        .map(User::from)
        .into_json()
}

#[put("/<id>", format = "json", data = "<user>")]
fn update(id: i64, user: Json<NewUser<'_>>, context: AuthContext) -> Result<Json<User>, HttpError> {
    user::find_user_by_id(&context, id)
        .map(User::from)
        .into_json()
}
