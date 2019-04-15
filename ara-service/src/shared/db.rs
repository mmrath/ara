use crate::shared::config::Database;
use ara_model::db::Connection;
use ara_model::db::ConnectionPool;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

/// Creates the database connection pool
pub fn establish_connection_pool(db_config: &Database) -> ConnectionPool {
    let database_url = &db_config.url;
    let manager = ConnectionManager::<Connection>::new(database_url.as_str());

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
