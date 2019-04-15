use ara_model::core::User;
use ara_model::db::{Connection, ConnectionPool, PooledConnection};
use ara_service::shared::{Context, PlainContext};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

pub struct AuthContext {
    // add user here
    db: PooledConnection,
    user: User,
}

pub struct UnauthContext {
    db: PooledConnection,
}

impl PlainContext for UnauthContext {
    fn db(&self) -> &Connection {
        &self.db
    }
}

impl PlainContext for AuthContext {
    fn db(&self) -> &Connection {
        &self.db
    }
}

impl Context for AuthContext {
    fn user(&self) -> &User {
        &self.user
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UnauthContext {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UnauthContext, ()> {
        let pool = request.guard::<State<'_, ConnectionPool>>()?;
        pool.get()
            .map(|conn| Outcome::Success(UnauthContext { db: conn }))
            .unwrap_or_else(|_e| Outcome::Failure((Status::ServiceUnavailable, ())))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthContext {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthContext, ()> {
        let pool = request.guard::<State<'_, ConnectionPool>>()?;

        let user = User {
            id: 0,
            username: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            phone_number: None,
            active: false,
            version: 0,
        };
        pool.get()
            .map(|conn| Outcome::Success(AuthContext { db: conn, user }))
            .unwrap_or_else(|_e| Outcome::Failure((Status::ServiceUnavailable, ())))
    }
}
