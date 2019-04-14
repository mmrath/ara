use ara_model::core::User;
use ara_model::db::Connection;

pub trait Context: UnauthContext {
    fn user(&self) -> &User;
}

pub trait UnauthContext {
    fn db(&self) -> &Connection;
}
