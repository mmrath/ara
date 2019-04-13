use std::ops::Deref;

use ara_model::db::{Connection, PooledConnection};

pub struct DB(pub PooledConnection);

impl Deref for DB {
    type Target = Connection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
