use crate::shared::config::Database;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use ara_model::db::Connection;
use ara_model::db::ConnectionPool;

/// Creates the database connection pool
pub fn establish_connection_pool(db_config: &Database) -> ConnectionPool {
    let database_url = &db_config.url;
    let manager = ConnectionManager::<Connection>::new(database_url.as_str());

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
