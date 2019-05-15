use tide::{self, response::{IntoResponse, Response}};

use ara_model::db::ConnectionPool;
use ara_common::context::{Context, UnauthContext};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: ConnectionPool,
}

pub trait TideContextExt {
    fn context(&mut self) -> impl Context;
    fn unauth_context(&mut self) -> impl UnauthContext;
}

impl<AppData> TideContextExt for tide::Context<AppData> {
    fn context(&mut self) -> impl Context {
        unimplemented!()
    }

    fn unauth_context(&mut self) -> impl UnauthContext {
        unimplemented!()
    }
}

pub struct AuthorizedContext {
    // add user here
    db: PooledConnection,
    user: User,
}

pub struct UnauthorizedContext {
    db: PooledConnection,
}