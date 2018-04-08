use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use futures::future::Future;
use chrono::{ Utc,NaiveDateTime };
use utils::token::verify_token;
use handler::index::State;
use model::db::ConnDsl;
use model::response::UserInfoMsgs;
use model::user::{ User, UserInfo };

impl Message for UserInfo {
    type Result = Result<UserInfoMsgs, Error>;
}

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
                                    Ok(httpcodes::HTTPOk.build().json(user_info)?)
                                },
                                Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                            }
                        }).responder()
                    },
                    Err(_) => {
                            req.state().db.send(UserInfo{user_id: "".to_string()})         
                                .from_err()
                                .and_then(|res| {
                                    match res {
                                        Ok(msg) => Ok(httpcodes::HTTPOk.build().json(msg)?),
                                        Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                                    }
                                }).responder()
                    },
            }
        };
        Ok(fut)
}

impl Handler<UserInfo> for ConnDsl {
    type Result = Result<UserInfoMsgs, Error>;
    fn handle(&mut self, user_info: UserInfo, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let user_id: i32 = user_info.user_id.parse().map_err(error::ErrorBadRequest)?;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let login_user =  users.filter(&id.eq(&user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
        match login_user {
            Some(login_user) => {
                    let current_user = User {
                            id: login_user.id,
                            email: login_user.email,
                            username: login_user.username,
                            password: login_user.password,
                            created_at : login_user.created_at,
                    };
                    Ok(UserInfoMsgs { 
                            status: 200,
                            message : "The  current_user info.".to_string(),
                            current_user: current_user,
                    })
            },
            None => {
                    let no_user = User {
                            id: 0,
                            email: "".to_owned(),
                            username: "".to_owned(),
                            password: "".to_owned(),
                            created_at: Utc::now().naive_utc(),
                    };
                    Ok(UserInfoMsgs { 
                            status: 400,
                            message : "error.".to_string(),
                            current_user: no_user,
                    })
            },
        }
    }
}