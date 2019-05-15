use crate::shared::json_map;
use ara_model::core::{read_notification, Notification, User};
use ara_model::db::Connection;
use diesel::query_dsl::RunQueryDsl;
use log::info;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};
use serde_json::{json, Map, Value};

static TEST_USERNAME: &str = "testuser";
static TEST_PASSWORD: &str = "password12";

// Get the tasks before making changes.
static EMAIL: &str = "john.doe@acme.org";
static PASSWORD: &str = "password12";
static FIRST_NAME: &str = "John";
static LAST_NAME: &str = "Doe";

#[test]
fn create_get_update_user() {
    let test = |client: Client, conn: &Connection| {
        let user_json = json!({
                "firstName": FIRST_NAME,
                "lastName": LAST_NAME,
                "email": EMAIL,
        });

        let resp = create_test_user(&client, &user_json);

        let user_id: i64 = resp.get("id").unwrap().as_i64().unwrap();

        let user = get_test_user(&client, user_id);

        assert_eq!(user.get("email").unwrap(), EMAIL);
        assert_eq!(user.get("firstName").unwrap(), FIRST_NAME);
        assert_eq!(user.get("lastName").unwrap(), LAST_NAME);
        assert_eq!(user.get("username").unwrap(), EMAIL);
        assert_eq!(user.get("locked").unwrap(), false);
        assert_eq!(user.get("activated").unwrap(), false);
        assert_eq!(user.get("version").unwrap(), 1);

        let resp = update_test_user(&client, user_id, &user_json);
    };
    run_test!(|client, conn| test(client, conn));
}

fn create_test_user(client: &Client, user_json: &Value) -> Map<String, Value> {
    let mut resp = client
        .post("/api/user/")
        .header(ContentType::JSON)
        .body(user_json.to_string())
        .dispatch();

    info!("Result is {:?}", resp);
    let resp_body = json_map(&mut resp);
    info!("Result is {:?}", resp);
    info!("Body is {:?}", resp_body);

    let email = user_json.get("email").unwrap().as_str().unwrap();
    let first_name = user_json.get("firstName").unwrap().as_str().unwrap();
    let last_name = user_json.get("lastName").unwrap().as_str().unwrap();

    assert_eq!(resp.status(), Status::Ok);
    assert_eq!(resp_body.get("email").unwrap(), email);
    assert_eq!(resp_body.get("firstName").unwrap(), first_name);
    assert_eq!(resp_body.get("lastName").unwrap(), last_name);
    assert_eq!(resp_body.get("username").unwrap(), email);
    assert_eq!(resp_body.get("locked").unwrap(), false);
    assert_eq!(resp_body.get("activated").unwrap(), false);
    assert_eq!(resp_body.get("version").unwrap(), 1);

    resp_body
}

fn get_test_user(client: &Client, user_id: i64) -> Map<String, Value> {
    let mut resp = client.get(format!("/api/user/{}", user_id)).dispatch();

    let resp_body = json_map(&mut resp);
    info!("Result is {:?}", resp);
    info!("Body is {:?}", resp_body);
    assert_eq!(resp.status(), Status::Ok);
    resp_body
}

fn update_test_user(client: &Client, user_id: i64, user_json: &Value) -> Map<String, Value> {
    let mut resp = client
        .put(format!("/api/user/{}", user_id))
        .header(ContentType::JSON)
        .body(user_json.to_string())
        .dispatch();

    let resp_body = json_map(&mut resp);
    info!("Result is {:?}", resp);
    info!("Body is {:?}", resp_body);
    assert_eq!(resp.status(), Status::Ok);
    resp_body
}
