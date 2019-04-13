#![feature(proc_macro_hygiene, decl_macro)]
#![deny(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
// To be removed after https://github.com/diesel-rs/diesel/issues/1785 released
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(inner_deref)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate err_derive;

pub mod core;
pub mod shared;
