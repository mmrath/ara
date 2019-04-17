use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use failure::Fail;

pub type Connection = PgConnection;
pub type ConnectionPool = Pool<ConnectionManager<Connection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

#[macro_export]
macro_rules! with_tx {
    ($conn:ident, $tx:block) => {
        let tm = conn.transaction_manager();
        tm.begin_transaction(conn).map_err(TxError::BeginFailed)?;

        let res = $tx;

        match res {
            Err(ref _e) => tm
                .rollback_transaction(conn)
                .map_err(TxError::RollbackFailed)?,
            Ok(_) => tm.commit_transaction(conn).map_err(TxError::CommitFailed)?,
        }
        res
    }
}


pub fn tx<T: Sized, E:From<failure::Error>, F: FnOnce(&Connection) -> Result<T, E>>(
    conn: &Connection,
    f: F,
) -> Result<T, E> {
    use diesel::connection::TransactionManager;
    use diesel::Connection;

    let tm = conn.transaction_manager();
    tm.begin_transaction(conn).map_err(|e|e.into())?;
    let res = f(conn);

    match res {
        Err(ref _e) => tm
            .rollback_transaction(conn)
            .map_err(|e|e.into())?,
        Ok(_) => tm.commit_transaction(conn).map_err(|e|e.into())?,
    }
    res
}

#[derive(Debug, Fail)]
pub enum TxError {
    #[fail(display = "Failed to start transaction")]
    BeginFailed(diesel::result::Error),
    #[fail(display = "Failed to rollback transaction")]
    RollbackFailed(diesel::result::Error),
    #[fail(display = "Failed to commit transaction")]
    CommitFailed(diesel::result::Error),
}
