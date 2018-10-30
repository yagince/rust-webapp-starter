pub mod article;
pub mod user;
pub mod home;
pub mod auth;

use actix_web::{ HttpResponse, dev::HttpResponseBuilder };

fn status(status: &i32) -> HttpResponseBuilder {
    match status {
        400 => HttpResponse::BadRequest(),
        _ => HttpResponse::Ok(),
    }
}
