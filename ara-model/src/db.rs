use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub type Connection = PgConnection;
pub type ConnectionPool = Pool<ConnectionManager<Connection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

pub fn tx<T: Sized, E: From<TxError>, F: FnOnce(&Connection) -> Result<T, E>>(
    conn: &Connection,
    f: F,
) -> Result<T, E> {
    use diesel::connection::TransactionManager;
    use diesel::Connection;

    let tm = conn.transaction_manager();
    tm.begin_transaction(conn).map_err(TxError::BeginFailed)?;
    let res = f(conn);

    match res {
        Err(ref _e) => tm
            .rollback_transaction(conn)
            .map_err(TxError::RollbackFailed)?,
        Ok(_) => tm.commit_transaction(conn).map_err(TxError::CommitFailed)?,
    }
    res
}

#[derive(Debug, Error)]
pub enum TxError {
    #[error(display = "Failed to start transaction")]
    BeginFailed(diesel::result::Error),
    #[error(display = "Failed to rollback transaction")]
    RollbackFailed(diesel::result::Error),
    #[error(display = "Failed to commit transaction")]
    CommitFailed(diesel::result::Error),
}
