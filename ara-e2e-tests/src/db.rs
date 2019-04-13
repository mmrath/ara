use postgres::params::{Builder, Host};
use postgres::{Connection, TlsMode};

/// Connects to DB and drops all test users and schemas so all test users and schemas can be
/// recreated
pub fn init_db(db_host: &str, username: &str, password: &str, db: &str) -> Connection {
    let params = Builder::new()
        .user(username, Some(password))
        .database(db)
        .build(Host::Tcp("localhost".to_owned()));

    // use a pool?
    let conn = Connection::connect(params, TlsMode::None).unwrap();

    drop_test_users_and_schemas(&conn, db);
    conn
}

/// Creates the test user and schema. Returns the URL of the DB
pub fn create_test_db(conn: &Connection, db: &str, i: u16) -> (String, Connection) {
    let user = format!("temp_test_user_{}", i);
    let schema = format!("temp_test_schema_{}", i);

    create_db_user(&conn, db, &user, &schema);

    let user_db_url = format!("postgres://{}:{}@localhost/{}", user, user, db);

    // use a pool?
    let conn = Connection::connect(user_db_url.as_ref(), TlsMode::None).unwrap();

    (user_db_url, conn)
}

fn drop_test_users_and_schemas(conn: &Connection, db: &str) {
    for i in 1..=10 {
        let schema = format!("temp_test_schema_{}", i);
        let user = format!("temp_test_user_{}", i);

        conn.execute(&format!("DROP SCHEMA IF EXISTS {}", schema), &[])
            .unwrap();

        // no unwrap because it may fail when user does not exist
        conn.execute(
            &format!("REVOKE ALL PRIVILEGES ON DATABASE {} FROM {}", db, user),
            &[],
        );
        conn.execute(&format!("DROP OWNED BY {}", user), &[]);

        conn.execute(&format!("DROP USER IF EXISTS {}", user), &[])
            .unwrap();
    }
}

/// Creates the user and schema
fn create_db_user(conn: &Connection, db: &str, user: &str, schema: &str) {
    conn.execute(
        format!(
            "CREATE USER {} WITH PASSWORD '{}' NOCREATEDB NOINHERIT",
            user, user
        )
        .as_ref(),
        &[],
    )
    .unwrap();

    conn.execute(format!("CREATE SCHEMA {}", schema).as_ref(), &[])
        .unwrap();

    conn.execute(
        format!("GRANT CONNECT ON DATABASE {} TO {}", db, user).as_ref(),
        &[],
    )
    .unwrap();

    conn.execute(
        format!("GRANT USAGE ON SCHEMA {} TO {}", schema, user).as_ref(),
        &[],
    )
    .unwrap();

    conn.execute(
        format!("ALTER ROLE {} SET search_path TO {}", user, schema).as_ref(),
        &[],
    )
    .unwrap();
}

pub fn clean_db(conn: &Connection) {
    conn.execute("TRUNCATE TABLE user_token CASCADE", &[])
        .unwrap();
    conn.execute("TRUNCATE TABLE app_user CASCADE", &[])
        .unwrap();
}
