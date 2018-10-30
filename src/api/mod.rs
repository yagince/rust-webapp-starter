pub mod article;
pub mod user;
pub mod home;
pub mod auth;

use actix_web::{
    FromRequest,
    HttpRequest,
    HttpResponse,
    dev::HttpResponseBuilder,
    error::{self, Error},
};

use share::common::{Claims, decode};

fn status(status: &i32) -> HttpResponseBuilder {
    match status {
        400 => HttpResponse::BadRequest(),
        _ => HttpResponse::Ok(),
    }
}

impl<S> FromRequest<S> for Claims {
    type Config = ();
    type Result = Result<Claims, Error>;
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        if let Some(header) = req.headers().get("Authorization") {
            let header_token = header.to_str().unwrap();
            let token = header_token[7..].to_string();

            decode(&token)
                .map(|x| x.claims)
                .map_err(|_| error::ErrorUnauthorized("Unauthorized"))
        } else {
            Err(error::ErrorUnauthorized("Unauthorized"))
        }
    }
}
