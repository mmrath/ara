use crate::context::TestContext;
use postgres::Connection;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::Stdio;

pub fn create_app_server() -> TestContext {
    env::set_var("RUST_BACKTRACE", "1");

    let host = "localhost";
    let username = "fina_test";
    let password = "password";
    let db = "fina_testdb";

    //let mail_api = crate::email::download_and_run_mail_server();
    let conn = crate::db::init_db(host, username, password, db);
    let (db_url, user_conn) = crate::db::create_test_db(&conn, db, 1);

    env::set_var("DATABASE_URL", db_url);
    let app_url = exe_bin();

    TestContext {
        app_url,
        conn: user_conn,
        //mail_api_url: mail_api,
    }
}

fn exe_bin() -> String {
    let work_dir = "..";
    let app_bin = "./target/debug/ara-rocket-app";
    let mut cmd = Command::new(app_bin);

    println!("Command is {:?}", cmd);

    let exe_path = Path::new("../target/debug/ara-rocket-app");
    if !exe_path.exists() {
        panic!("Executable not found at {}", exe_path.to_str().unwrap())
    }

    let mut process = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(work_dir)
        .spawn()
        .expect("server failed to start");

    println!("Server process {}", process.id());

    let stdout = process.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::with_capacity(100, stdout);
    let stdout_lines = stdout_reader.lines();

    let mut url = None;
    for line in stdout_lines {
        match line {
            Ok(text) => {
                println!("{}", text);
                if let Some(i) = text.find("Rocket has launched from") {
                    let url_idx = text.find("http://").unwrap();
                    url = Some(text.split_at(url_idx).1.to_owned());
                    break;
                }
            }
            Err(e) => {
                println!("Read error: {}", e);
            }
        }
    }
    println!("Rocket url {:?}", url);

    url.unwrap()
}
