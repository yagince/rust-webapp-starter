use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::Future;
use jwt::{decode, Header, Algorithm, Validation};

use share::common::Claims;
use model::user::{UserInfo, UserDelete, UserUpdate};
use share::state::AppState;


pub fn user_info(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
            let header = req.headers().get("Authorization").unwrap();
            let header_token = header.to_str().unwrap();
            let token = header_token[7..].to_string();
            let key = "secret";
            let validation = Validation {
                validate_exp: false,
                ..Default::default()
            };
            match decode::<Claims>(&token, key.as_ref(), &validation) {
            Ok(token_data) =>{
                req.state().db.send(UserInfo{user_id: token_data.claims.user_id.to_string()})
                    .from_err()
                    .and_then(|res| {
                        match res {
                            Ok(user_info) => {
                                Ok(HttpResponse::Ok().json(user_info))
                            },
                            Err(_) => Ok(HttpResponse::InternalServerError().into())
                        }
                    }).responder()
            },
            Err(_) => {
                req.state().db.send(UserInfo{user_id: "".to_string()})
                    .from_err()
                    .and_then(|res| {
                        match res {
                            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                            Err(_) => Ok(HttpResponse::InternalServerError().into())
                        }
                    }).responder()
            }
        }
}

pub fn user_delete(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
        let header = req.headers().get("Authorization").unwrap();
        let header_token = header.to_str().unwrap();
        let token = header_token[7..].to_string();
        let key = "secret";
        let validation = Validation {
            validate_exp: false,
            ..Default::default()
        };
        match decode::<Claims>(&token, key.as_ref(), &validation) {
            Ok(token_data) =>{
                req.state().db.send(UserDelete{user_id: token_data.claims.user_id.to_string()})
                    .from_err()
                    .and_then(|res| {
                        match res {
                            Ok(user_delete) => {
                                Ok(HttpResponse::Ok().json(user_delete))
                            },
                            Err(_) => Ok(HttpResponse::InternalServerError().into())
                        }
                    }).responder()
            },
            Err(_) => {
                req.state().db.send(UserDelete{user_id: "".to_string()})
                    .from_err()
                    .and_then(|res| {
                        match res {
                            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                            Err(_) => Ok(HttpResponse::InternalServerError().into())
                        }
                    }).responder()
            }
        }
}

pub fn user_update((user_update, state): (Json<UserUpdate>, State<AppState>)) -> FutureResponse<HttpResponse> {
        state.db.send(UserUpdate{ 
                user_id: user_update.user_id,
                newname: user_update.newname.clone(),
                newmail: user_update.newmail.clone(),
                newpassword: user_update.newpassword.clone(),
                confirm_newpassword: user_update.confirm_newpassword.clone(),
            })         
            .from_err()
            .and_then(|res| {
                    match res {
                        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                        Err(_) => Ok(HttpResponse::InternalServerError().into())
                    }
            }).responder()
}