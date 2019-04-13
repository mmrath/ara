use diesel::result::Error as DieselError;

#[derive(Debug, Error)]
pub enum DbError {
    #[error(display = "Internal error: {}", _0)]
    DieselError(DieselError),

    #[error(display = "No data found")]
    NotFound,

    #[error(display = "Incorrect result size: expected:{} found:{}", _0, _1)]
    IncorrectResultSize(usize, usize),

    #[error(display = "Transaction error")]
    TxError,

    #[error(display = "Unknown error")]
    __NonExhaustive,
}

impl From<DieselError> for DbError {
    fn from(err: DieselError) -> Self {
        DbError::DieselError(err)
    }
}
