#![allow(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate num_cpus;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate chrono;
extern crate bcrypt;
extern crate http;
extern crate ring;
extern crate data_encoding;
extern crate regex;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use actix::*;
use actix_web::*;
use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager };

mod api;
mod handler;
mod model;
mod utils;

use model::db::ConnDsl;
// use model::pg::PoolPg;
use utils::cors;
use handler::index::{ State, home, path };
use handler::auth::{ signup, signin };
use api::article::{ article,article_list, article_new };
use api::user::user_info;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = actix::System::new("webapp");

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder().build(manager).expect("Failed to create pool.");
    let addr = SyncArbiter::start( num_cpus::get() * 4, move || { ConnDsl(conn.clone()) });
    // let addr_pg = SyncArbiter::start( num_cpus::get() * 4, || PoolPg::new());
    HttpServer::new(
        move || Application::with_state(State{
            db: addr.clone(),
            // db_pg:addr_pg.clone(),
        })
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.h(home))
            .resource("/a/{tail:.*}", |r| r.h(path))
            .resource("/user/signup", |r| {
                cors::options().register(r);
                r.method(Method::POST).h(signup);
            })
            .resource("/user/signin", |r| {
                cors::options().register(r);
                r.method(Method::POST).h(signin);
            })
            .resource("/api/article_list", |r| {
                cors::options().register(r);
                r.method(Method::GET).h(article_list);
            })
            .resource("/api/article_new", |r| {
                cors::options().register(r);
                r.method(Method::POST).h(article_new);
            })
            .resource("/api/user_info", |r| {
                cors::options().register(r);
                r.method(Method::GET).h(user_info);
            })
            .resource("/api/{article_id}", |r| {
                cors::options().register(r);
                r.method(Method::GET).h(article);
            })
            .handler("/", fs::StaticFiles::new("public", true)))
        .bind("127.0.0.1:8000").unwrap()
        .shutdown_timeout(2)
        .start();

    let _ = sys.run();
}