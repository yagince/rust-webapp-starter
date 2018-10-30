use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::Future;

use share::common::{ Claims, self };
use model::user::{UserInfo, UserDelete, UserUpdate};
use share::state::AppState;

pub fn user_info((req, claims): (HttpRequest<AppState>, Claims)) -> FutureResponse<HttpResponse> {
    req.state().db.send(UserInfo{user_id: claims.user_id.to_string()})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(user_info) => {
                    Ok(HttpResponse::Ok().json(user_info))
                },
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_delete((req, claims): (HttpRequest<AppState>, Claims)) -> FutureResponse<HttpResponse> {
    req.state().db.send(UserDelete{user_id: claims.user_id.to_string()})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(user_delete) => {
                    Ok(HttpResponse::Ok().json(user_delete))
                },
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn user_update((user_update, state, claims): (Json<UserUpdate>, State<AppState>, Claims)) -> FutureResponse<HttpResponse> {
    state.db.send(UserUpdate{
        user_id: user_update.user_id,
        newname: user_update.newname.clone(),
        newmail: user_update.newmail.clone(),
        newpassword: user_update.newpassword.clone(),
        confirm_newpassword: user_update.confirm_newpassword.clone(),
    }).from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}
