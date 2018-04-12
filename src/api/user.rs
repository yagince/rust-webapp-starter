use actix_web::{HttpMessage, HttpRequest, HttpResponse, error, Error, AsyncResponder};
use futures::future::Future;
use utils::token::verify_token;

use model::user::{UserInfo, UserDelete, UserUpdate};
use api::index::State;


pub fn user_info(req: HttpRequest<State>) -> Result<Box<Future<Item=HttpResponse, Error=Error>>, Error> {
        let header_token = req.headers().get("Authorization").ok_or_else(|| error::ErrorForbidden("Authorization is required"))?;
        let token = header_token.to_str().map_err(error::ErrorForbidden)?;
        let fut = {
            let user_id = token[7..].to_string();
            match verify_token(&user_id) {
                Ok(user_id) => {
                    req.state().db.send(UserInfo{user_id})         
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
                    },
            }
        };
        Ok(fut)
}

pub fn user_delete(req: HttpRequest<State>) -> Result<Box<Future<Item=HttpResponse, Error=Error>>, Error> {
        let header_token = req.headers().get("Authorization").ok_or_else(|| error::ErrorForbidden("Authorization is required"))?;
        let token = header_token.to_str().map_err(error::ErrorForbidden)?;
        let fut = {
            let user_id = token[7..].to_string();
            match verify_token(&user_id) {
                Ok(user_id) => {
                    req.state().db.send(UserDelete{user_id})         
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
                    },
            }
        };
        Ok(fut)
}

pub fn user_update(req: HttpRequest<State>) -> Result<Box<Future<Item=HttpResponse, Error=Error>>, Error> {
    let fut = req.clone().json()                     
                .from_err()
                .and_then(move |user_update: UserUpdate| {  
                        req.state().db.send(UserUpdate{ 
                            user_id: user_update.user_id,
                            newname: user_update.newname,
                            newmail: user_update.newmail,
                            newpassword: user_update.newpassword,
                            confirm_newpassword: user_update.confirm_newpassword,
                        })         
                        .from_err()
                        .and_then(|res| {
                            match res {
                                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                                Err(_) => Ok(HttpResponse::InternalServerError().into())
                            }
                        })
                    }).responder();
    Ok(fut)
}