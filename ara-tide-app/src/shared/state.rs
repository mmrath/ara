use futures::future;
use tide::Extract;
use tide::IntoResponse;
use tide::Request;
use tide::Response;
use tide::RouteMatch;

use ara_model::db::ConnectionPool;

use crate::shared::db::DB;
use tide::configuration::Store;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: ConnectionPool,
}

impl Extract<AppState> for DB {
    type Fut = future::Ready<Result<Self, Response>>;

    fn extract(
        data: &mut AppState,
        _req: &mut Request,
        _params: &Option<RouteMatch<'_>>,
        _store: &Store,
    ) -> Self::Fut {
        let pool = &data.db_pool;
        pool.get()
            .map(|conn| future::ok(DB(conn)))
            .unwrap_or_else(|_e| future::err(http::StatusCode::SERVICE_UNAVAILABLE.into_response()))
    }
}
