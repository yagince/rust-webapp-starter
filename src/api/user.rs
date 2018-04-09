use actix_web::*;
use futures::future::Future;
use utils::token::verify_token;

use model::user::UserInfo;
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


