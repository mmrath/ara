use crate::core::user;
use ara_common::config::AppConfig;
use ara_model::db::PooledConnection;
use log::info;
use rocket::config::{Config, Environment};
use rocket::Rocket;
use rocket_contrib::json::JsonValue;

fn setup_logger() -> Result<(), failure::Error> {
    log4rs::init_file("res/config/log4rs.yaml", Default::default()).unwrap();
    Ok(())
}

pub fn run(config_dir: &str, env: &str) {
    setup_logger().unwrap();
    AppConfig::init(config_dir, env);
    let config = AppConfig::get();
    rocket(config).0.launch();
}

pub fn rocket(config: &AppConfig) -> (Rocket, PooledConnection) {
    info!("Launching server with config {}", json!(config).to_string());
    let pool = ara_common::db::establish_connection_pool(config.database.url.as_str());
    let conn = pool.get().expect("database connection for testing");

    let port = config.server.port;

    let config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(port)
        .finalize()
        .expect("Rocket config failed");

    let rocket = rocket::custom(config)
        .manage(pool)
        .register(catchers![not_found])
        .mount("/api/user", user::web::routes());

    (rocket, conn)
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
