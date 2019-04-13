use failure::{Error, Fail};
use lazy_static::lazy_static;
use postgres::{Connection, TlsMode};
use serde_json::{self, json, json_internal, Map, Value};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

/*
pub struct TestContext {
    pub process: Child,
    pub base_url: String,
    pub db: Connection,
}


impl TestContext {
    pub fn new() -> Self {
        let mut cmd = Command::new("../target/debug/ara_tide_app");

        println!("Command is {:?}", cmd);

        let mut process = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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
                    if let Some(i) = text.find("Server is listening on") {
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

        // use a pool?
        let conn = Connection::connect(
            std::env::var("DATABASE_URL").expect("DB URL not found"),
            TlsMode::None,
        )
        .unwrap();

        super::clean_db(&conn);
        super::download_and_run_mail_slurper();
        clean_mailbox();

        TestContext {
            process,
            base_url: url.unwrap(),
            db: conn,
        }
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn url(&self, path: &str) -> String {
        if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        }
    }

    pub fn db(&self) -> &Connection {
        &self.db
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        self.process.kill().unwrap();
    }
}

*/


