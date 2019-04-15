use ara_model::db::{Connection, PooledConnection};
use diesel::connection::SimpleConnection;
use failure::Error;
use log::info;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use rocket::local::LocalResponse;
use rocket::Rocket;
use serde_json::{Map, Value};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// We use a lock to synchronize between tests so DB operations don't collide.
// For now. In the future, we'll have a nice way to run each test in a DB
// transaction so we can regain concurrency.
pub static DB_LOCK: Mutex<()> = Mutex::new(());

static INIT_TEST_APP: OnceCell<()> = OnceCell::INIT;

#[macro_export]
macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        crate::shared::init_test();
        let _test_lock = crate::shared::DB_LOCK.lock();
        let _test_execution_start = std::time::Instant::now();
        let config = ara_service::shared::config::AppConfig::get();
        info!("Loaded config");
        let (rocket, conn) = ara_admin_api::rocket(config);
        info!(
            "Elapsed time at  building rocket {:?}",
            _test_execution_start.elapsed()
        );
        info!("Setup rocket");
        crate::shared::clean_seed_db(&conn);
        info!(
            "Elapsed time at clean db {:?}",
            _test_execution_start.elapsed()
        );
        info!("Cleaned database");
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = &conn;
        {
            $block
        }
        info!(
            "Total time for testing {:?}",
            _test_execution_start.elapsed()
        );
    }};
}

pub fn init_test() {
    INIT_TEST_APP.get_or_init(|| {
        log4rs::init_file("../res/config/log4rs-test.yaml", Default::default()).unwrap();
        ara_service::shared::config::AppConfig::init("../res/config", "local-test");
        ()
    });
}

pub fn json_value(resp: &mut LocalResponse) -> Value {
    let json_value =
        serde_json::from_str(resp.body().unwrap().into_string().unwrap().as_str()).unwrap();
    json_value
}

pub fn json_map(resp: &mut LocalResponse) -> Map<String, Value> {
    let json_res = json_value(resp);
    match json_res {
        Value::Object(map) => map,
        _ => {
            info!("Not json object: {:?}", json_res.to_string());
            panic!("Not a json object")
        }
    }
}

pub fn clean_seed_db(conn: &Connection) {
    use diesel::query_dsl::RunQueryDsl;
    run_sql_from_file(conn, Path::new("../res/scripts/db/clean.sql"));
    run_sql_from_file(conn, Path::new("../res/scripts/db/seed.sql"));
}

fn run_sql_from_file(conn: &SimpleConnection, path: &Path) {
    let mut sql = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut sql).unwrap();
    conn.batch_execute(&sql).unwrap();
}
