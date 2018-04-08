use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use utils::token;
use chrono::{ Utc,NaiveDateTime };
use bcrypt::{DEFAULT_COST, hash, verify};

use model::user::{User, NewUser, SignupUser, SigninUser};
use model::response::{Msgs, SigninMsgs};
use model::db::ConnDsl;


impl Message for SignupUser {
    type Result = Result<Msgs, Error>;
}

impl Message for SigninUser {
    type Result = Result<SigninMsgs, Error>;
}

impl Handler<SignupUser> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, signup_user: SignupUser, _: &mut Self::Context) -> Self::Result {
        if &signup_user.password == &signup_user.confirm_password {
                use utils::schema::users::dsl::*;
                let hash_password = match hash(&signup_user.password, DEFAULT_COST) {
                    Ok(h) => h,
                    Err(_) => panic!()
                };
                let new_user = NewUser {
                    email: &signup_user.email,
                    username: &signup_user.username,
                    password: &hash_password,
                    created_at: Utc::now().naive_utc(),
                };
                let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
                diesel::insert_into(users).values(&new_user).execute(conn).map_err(error::ErrorInternalServerError)?;
                Ok(Msgs { 
                        status: 200,
                        message : "Successful Signup.".to_string(),
                })
        }else{
            Ok(Msgs { 
                    status: 400,
                    message : "failed Signup.".to_string(),
            })
        }
    }
}

impl Handler<SigninUser> for ConnDsl {
    type Result = Result<SigninMsgs, Error>;

    fn handle(&mut self, signin_user: SigninUser, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let login_user =  users.filter(&username.eq(&signin_user.username)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let no_user = User {
            id: 0,
            email: "".to_owned(),
            username: "".to_owned(),
            password: "".to_owned(),
            created_at: Utc::now().naive_utc(),
        };
        match login_user {
            Some(login_user) => {
                match verify(&signin_user.password, &login_user.password) {
                    Ok(valid) => {
                        let user_id = login_user.id.to_string();
                        let token = token::generate_token(user_id).unwrap();
                        let the_user = User {
                            id: login_user.id,
                            email: login_user.email.clone(),
                            username: login_user.username.clone(),
                            password: login_user.password.clone(),
                            created_at : login_user.created_at.clone(),
                        };
                        Ok(SigninMsgs { 
                            status: 200,
                            token: token,
                            signin_user: the_user,
                            message: "Succesfully signin.".to_string(),
                        })
                    },
                    Err(_) => {
                        Ok(SigninMsgs { 
                            status: 400,
                            token: "".to_owned(),
                            signin_user: no_user,
                            message: "Incorrect Password.".to_string(),
                        })
                    },
                }
            },
            None => {
                Ok(SigninMsgs { 
                    status: 400,
                    token: "".to_owned(),
                    signin_user: no_user,
                    message: "Signin failure.".to_string(),
                })
            }
        }
    }
}
