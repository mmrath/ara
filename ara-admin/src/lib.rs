#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub use self::app::*;

mod app;
pub(crate) mod service;
mod shared;
mod web;
