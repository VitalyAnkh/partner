#![allow(warnings)]

extern crate actix;
extern crate actix_web;
extern crate actix_redis;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate crypto;

mod common;
mod controllers;
mod models;

use std::sync::Arc;
use actix_web::{server, App, http, middleware};
use actix_web::middleware::session::SessionStorage;
use actix_redis::RedisSessionBackend;
use controllers::user;
use common::state::AppState;

fn main() {

    let config = dotenv::var("CONFIG").expect("CONFIG must be set in .env file");

    if config == "dev" {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let actix_sys = actix::System::new("partner");

    server::new(|| {
        App::with_state(AppState::new())
            .middleware(middleware::Logger::default())
            .middleware(SessionStorage::new(
                RedisSessionBackend::new("127.0.0.1:6379", &[0;32])
            ))
            .prefix("/api")
            .resource("/register", |r| {
                r.method(http::Method::POST).with2(user::register)
            })
            .resource("/login", |r| {
                r.method(http::Method::POST).with2(user::login)
            })
            .resource("/update", |r| {
                r.method(http::Method::PUT).with2(user::update)
            })
            .resource("/delete", |r| {
                r.method(http::Method::DELETE).with2(user::delete)
            })
            .resource("/reset-password", |r| {
                r.method(http::Method::PUT).f(user::reset_password)
            })
        })
        .bind("127.0.0.1:8888")
        .expect("can't bind to port 8888")
        .start();

    println!("server is listening on port 8888 !");

    actix_sys.run();
}
