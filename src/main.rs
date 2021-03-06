#![allow(warnings)]

extern crate actix;
extern crate actix_web;
extern crate actix_redis;
#[macro_use]
extern crate redis_async;
extern crate futures;
extern crate cookie;
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
extern crate toml;
extern crate meval;
extern crate env_logger;
extern crate log4rs;

mod common;
mod controllers;
mod models;

use actix_web::{server, App, http, middleware, pred};
use actix_web::http::{header};
use actix_web::middleware::{session::SessionStorage, cors::Cors};
use actix_redis::RedisSessionBackend;
use chrono::Duration;

use controllers::user;
use controllers::work_record;
use controllers::error;
use common::state::AppState;
use common::lazy_static::CONFIG;
use common::middlewares::{Remember, MarkLoginState};
use common::filters::CheckLogin;

fn main() {

    let app_env = dotenv::var("APP_ENV").expect("APP_ENV must be set in .env file");

    if app_env == "dev" {
        
        std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
        std::env::set_var("RUST_BACKTRACE", "1");

        env_logger::init();
    } else {

        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    }

    let actix_sys = actix::System::new(&*CONFIG.app.name);

    server::new(|| {
            vec![
                App::with_state(AppState::new(&*CONFIG.redis.url))
                .middleware(middleware::Logger::default())
                .middleware(Remember)
                .middleware(SessionStorage::new(
                    RedisSessionBackend::new(&*CONFIG.redis.url, &[0;32])
                                    .ttl(CONFIG.redis.ttl as u16)
                                    .cookie_max_age(Duration::seconds(CONFIG.cookie.max_age as i64))
                ))
                .middleware(MarkLoginState)
                .prefix("/api")
                .configure(|app| {
                    Cors::for_app(app)
                    .allowed_origin(&CONFIG.app.allowed_origin)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![header::ORIGIN, header::ACCEPT, header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(CONFIG.app.cache_max_age as usize)
                    .resource("/register", |r| {
                        r.method(http::Method::POST).with(user::register)
                    })
                    .resource("/login", |r| {
                        r.method(http::Method::POST).with(user::login)
                    })
                    .resource("/logout", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Get())
                        .f(user::logout);

                        r.f(error::handle_error);
                    })
                    .resource("/user/update", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Put())
                        .with(user::update);

                        r.f(error::handle_error);
                    })
                    .resource("/user/delete", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Delete())
                        .with(user::delete);

                        r.f(error::handle_error);
                    })
                    .resource("/modify-password", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Put())
                        .with(user::modify_password);

                        r.f(error::handle_error);
                    })
                    .resource("/work-record/create", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Post())
                        .with(work_record::create);

                        r.f(error::handle_error);
                    })
                    .resource("/work-record/update", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Put())
                        .with(work_record::update);

                        r.f(error::handle_error);
                    })
                    .resource("/work-record/get-records", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Post())
                        .with(work_record::get_records);

                        r.f(error::handle_error);
                    })
                    .resource("/work-record/get-record", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Post())
                        .with(work_record::get_record);

                        r.f(error::handle_error);
                    })
                    .resource("/work-record/delete", |r| {
                        r.route()
                        .filter(CheckLogin)
                        .filter(pred::Delete())
                        .with(work_record::delete);

                        r.f(error::handle_error);
                    })
                    .register()
                })
                .boxed(),

                App::new().resource("{tail:.*}", |r| {
                    r.f(error::not_found)
                })
                .boxed()
            ]
        })
        .bind(&format!("{}:{}", CONFIG.app.host, CONFIG.app.port))
        .expect(&format!("can't bind to port {}", CONFIG.app.port))
        .start();

    println!("{}", format!("server is listening on port {} !", CONFIG.app.port));

    actix_sys.run();
}
