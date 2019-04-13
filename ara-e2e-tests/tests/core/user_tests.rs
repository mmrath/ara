//use user::core::{self, fetch_mails, TestContext};
use reqwest::StatusCode;
use serde_json::{self, json, json_internal, Map, Value};
use std::collections::HashMap;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[test]
fn user_sign_up() {
    let _ = env_logger::try_init();
    ::std::env::set_var("RUST_LOG", "info,cargo=WARN,ara_tide_app=DEBUG");
    ::std::env::set_var("RUST_BACKTRACE", "full");

    let tcx = ara_tests::app::create_app_server();
    //std::thread::sleep(std::time::Duration::from_millis(60000));
    let client = reqwest::Client::new();
    let mut result = client
        .post(&tcx.url("/api/user/signup"))
        .json(&json!({
           "firstName": "John",
           "lastName": "Doe",
           "email": "john.doe@acme.org",
           "password": "h@rdToGu3$s"
        }))
        .send()
        .unwrap();

    assert_eq!(result.status(), StatusCode::OK);
    let user: Map<String, Value> = result.json().unwrap();
    assert_eq!(user.get("firstName").unwrap(), "John");

    let login_data = &json!({
       "username": "john.doe@acme.org",
       "password": "h@rdToGu3$s"
    });
    let mut login_res = client
        .post(&tcx.url("/api/user/login"))
        .json(login_data)
        .send()
        .unwrap();

    let login_res_body: Map<String, Value> = login_res.json().unwrap();

    assert_eq!(login_res.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(login_res_body.get("error").unwrap(), "LoginError");
    assert_eq!(login_res_body.get("kind").unwrap(), "AccountNotActivated");

    let emails = tcx.fetch_mails();

    let body = emails
        .get("mailItems")
        .unwrap()
        .as_array()
        .unwrap()
        .get(0)
        .unwrap()
        .get("body")
        .unwrap()
        .as_str()
        .unwrap();

    let start_bytes = body.find("/api/user/activate/").unwrap();
    let mut activation_url = &body[start_bytes..start_bytes + 51];

    println!("{}", activation_url);

    let mut act_res = client.get(&tcx.url(activation_url)).send().unwrap();

    assert_eq!(act_res.status(), StatusCode::OK);

    let mut login_res = client
        .post(&tcx.url("/api/user/login"))
        .json(login_data)
        .send()
        .unwrap();

    let login_res_body: Map<String, Value> = login_res.json().unwrap();

    assert_eq!(login_res.status(), StatusCode::OK);
}
