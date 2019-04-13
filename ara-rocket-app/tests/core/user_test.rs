use crate::shared::json_map;
use diesel::query_dsl::RunQueryDsl;
use log::info;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};
use ara_model::core::{read_notification, Notification, User};
use ara_model::db::Connection;
use serde_json::json;

static TEST_USERNAME: &str = "testuser";
static TEST_PASSWORD: &str = "password12";

// Get the tasks before making changes.
static EMAIL: &str = "john.doe@acme.org";
static PASSWORD: &str = "password12";
static FIRST_NAME: &str = "John";
static LAST_NAME: &str = "Doe";

#[test]
fn sign_up_and_activation_and_login() {
    let test = |client: Client, conn: &Connection| {
        verify_signup(&client, conn);
        let mut login_res = login(&client, conn);
        assert_eq!(login_res.status(), Status::Unauthorized);

        //let notf = retrieve_notification(conn);
        let login_res_body = json_map(&mut login_res);

        assert_eq!(login_res_body.get("error").unwrap(), "LoginError");
        assert_eq!(login_res_body.get("kind").unwrap(), "AccountNotActivated");
    };
    run_test!(|client, conn| test(client, conn));
}

#[test]
fn sign_up_and_login_without_activation() {
    let test = |client: Client, conn: &Connection| {
        verify_signup(&client, conn);
        let mut login_res = login(&client, conn);

        assert_eq!(login_res.status(), Status::Unauthorized);

        let login_res_body = json_map(&mut login_res);

        assert_eq!(login_res_body.get("error").unwrap(), "LoginError");
        assert_eq!(login_res_body.get("kind").unwrap(), "AccountNotActivated");
    };
    run_test!(|client, conn| test(client, conn));
}

fn verify_signup(client: &Client, conn: &Connection) {
    // Issue a request to insert a new task.
    let response = client
        .post("/api/user/signup")
        .header(ContentType::JSON)
        .body(
            json!({
                    "firstName": FIRST_NAME,
                    "lastName": LAST_NAME,
                    "email": EMAIL,
                    "password": PASSWORD
            })
            .to_string(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let user = User::find_by_username(conn, EMAIL).unwrap().unwrap();
    assert_eq!(user.email, EMAIL);
    assert_eq!(user.activated, false);
    assert_eq!(user.locked, false);
}

fn login<'a>(client: &'a Client, conn: &'a Connection) -> LocalResponse<'a> {
    let login_data = &json!({
        "username": EMAIL,
        "password": PASSWORD
    });

    let login_res = client
        .post("/api/user/login")
        .header(ContentType::JSON)
        .body(login_data.to_string())
        .dispatch();

    login_res
}

fn test_user_login<'a>(client: &'a Client, conn: &'a Connection) -> LocalResponse<'a> {
    let login_data = &json!({
        "username": TEST_USERNAME,
        "password": TEST_PASSWORD
    });

    let login_res = client
        .post("/api/user/login")
        .header(ContentType::JSON)
        .body(login_data.to_string())
        .dispatch();

    login_res
}

#[test]
fn login_and_change_password() {
    let test = |client: Client, conn: &Connection| {
        let mut login_res = test_user_login(&client, conn);

        assert_eq!(login_res.status(), Status::Ok);

        let login_res_body = json_map(&mut login_res);
        info!("Result is {:?}", login_res);
        info!("Body is {:?}", login_res_body);

        assert_eq!(login_res_body.get("token").is_some(), true);
    };
    run_test!(|client, conn| test(client, conn));
}
