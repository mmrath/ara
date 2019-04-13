use postgres::Connection;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub struct TestContext {
    pub(crate) app_url: String,
    pub(crate) conn: Connection,
}

impl TestContext {
    pub fn url(&self, path: &str) -> String {
        if path.starts_with('/') {
            format!("{}{}", self.app_url, path)
        } else {
            format!("{}/{}", self.app_url, path)
        }
    }
    pub fn fetch_mails(&self) -> Map<String, Value> {
        Map::new()
    }
}
