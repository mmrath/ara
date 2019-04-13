use ara_model::core::User;
use ara_model::db::Connection;

pub trait Context: PlainContext {
    fn user(&self) -> &User;
}

pub trait PlainContext {
    fn db(&self) -> &Connection;
}
