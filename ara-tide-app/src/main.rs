#![feature(async_await, futures_api)]

use ara_common::config::AppConfig;
use ara_model::db::Connection;
use chrono;
use diesel_migrations::{run_pending_migrations_in_directory, setup_database};
use fern;
use log;
use std::io::stdout;

pub mod core;
pub mod shared;

use crate::core::user;

/*
fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    //log4rs::init_file("resources/config/log4rs.yaml", Default::default()).unwrap();

    setup_logger().unwrap();
    let config = AppConfig::new().expect("Failed to load config");

    let pool = ara_service::shared::db::establish_connection_pool(&config.database);

    // block to drop connection after migration
    {
        let conn = pool.get().unwrap();
        migrate_db(&conn);
    }

    let state = crate::shared::state::AppState { db_pool: pool };

    let mut app = tide::App::new(state);

    app.at("/").nest(|router| {
        router.at("/").get(async || "Hello");
        router.at("/api").nest(|router| {
            router.at("/user").nest(|r| {
                r.at("/signup").post(user::sign_up);
                r.at("/login").post(user::login);
                r.at("/activate/{}").post(user::activate);
                r.at("/password-reset/init").post(user::password_reset_init);
                r.at("/password-reset/finish")
                    .post(user::password_reset_finish);
            });
            router.at("/role").nest(|r| {
                r.at("/{}").get(role::find);
                r.at("/{}").delete(role::delete);
                // r.at("/").post(role::create);
                r.at("/").put(role::update);
            });
        });
    });

    app.serve();
}

fn migrate_db(conn: &Connection) {
    setup_database(conn).expect("Failed to setup DB");
    run_pending_migrations_in_directory(conn, "./res/migrations".as_ref(), &mut stdout())
        .expect("Failed to run pending migration");
}

*/

fn main() {}
