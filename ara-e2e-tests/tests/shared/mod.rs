//mod server;

use parking_lot::Mutex;
use postgres::Connection;

//pub use self::server::*;
use std::io::BufRead;
use std::io::BufReader;

static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! run_test {
    (| $client:ident, $conn:ident | $block:expr) => {{
        ::std::env::set_var(
            "DATABASE_URL",
            "postgres://billac:billac@localhost/billacdb",
        );
        let _lock = DB_LOCK.lock();
        let (rocket, db) = ara_app_lib::rocket();
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");
        $block
    }};
}

#[cfg(test)]
pub fn clean_db(conn: &Connection) {
    conn.execute("TRUNCATE TABLE user_token CASCADE", &[])
        .unwrap();

    conn.execute("TRUNCATE TABLE app_user CASCADE", &[])
        .unwrap();
}
