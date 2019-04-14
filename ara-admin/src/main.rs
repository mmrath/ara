#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate ara_admin;

fn main() {
    let config_dir = "res/config";
    let env = "local";
    ara_admin::run(config_dir, env);
}
