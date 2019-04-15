use ara_model::db::Connection;
use ara_model::db::ConnectionPool;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

/// Creates the database connection pool
pub fn establish_connection_pool(db_url: &str) -> ConnectionPool {
    let manager = ConnectionManager::<Connection>::new(db_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
