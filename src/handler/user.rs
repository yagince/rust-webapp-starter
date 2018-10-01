use diesel::{self,sql_query,RunQueryDsl,QueryDsl,ExpressionMethods};
use actix_web::{actix::Handler, error,Error};
use chrono::Utc;
use bcrypt::{DEFAULT_COST, hash, verify};
use jwt::{encode, Header, Algorithm};

use model::user::{User, NewUser, SignupUser, SigninUser, UserInfo, UserUpdate, UserDelete};
use model::response::{Msgs, SigninMsgs, UserInfoMsgs};
use model::db::ConnDsl;
use share::common::Claims;
use model::response::MyError;

impl Handler<SignupUser> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, signup_user: SignupUser, _: &mut Self::Context) -> Self::Result {
        if &signup_user.password == &signup_user.confirm_password {
                use share::schema::users::dsl::*;
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
        use share::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let login_user =  users.filter(&username.eq(&signin_user.username)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let no_user = User::new();
        match login_user {
            Some(login_user) => {
                match verify(&signin_user.password, &login_user.password) {
                    Ok(valid) => {
                        let key = "secret";
                        let claims = Claims {
                            user_id: login_user.id.to_string(),
                        };
                        let token = match encode(&Header::default(), &claims, key.as_ref()) {
                            Ok(t) => t,
                            Err(_) => panic!() // in practice you would return the error
                        };
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

impl Handler<UserInfo> for ConnDsl {
    type Result = Result<UserInfoMsgs, Error>;

    fn handle(&mut self, user_info: UserInfo, _: &mut Self::Context) -> Self::Result {
        use share::schema::users::dsl::*;
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
                    let no_user = User::new();
                    Ok(UserInfoMsgs { 
                            status: 400,
                            message : "error.".to_string(),
                            current_user: no_user,
                    })
            },
        }
    }
}

impl Handler<UserDelete> for ConnDsl {
    type Result = Result<Msgs, MyError>;

    fn handle(&mut self, user_delete: UserDelete, _: &mut Self::Context) -> Self::Result {
        use share::schema::users::dsl::*;
        println!("============{:?}============", user_delete.user_id);
        let user_id: i32 = user_delete.user_id.parse().unwrap();
        let conn = &self.0.get().unwrap();
        let login_user = diesel::delete(users.filter(&id.eq(&user_id))).execute(conn);
        match login_user {
            Ok(Msgs) => Ok(Msgs{
                                status: 200,
                                message : "delete  loginuser success.".to_string(),
                        }),
            Ok(_) => Err(MyError::NotFound),
            Err(_) => Err(MyError::DatabaseError),
        }
    }
}

impl Handler<UserUpdate> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, user_update: UserUpdate, _: &mut Self::Context) -> Self::Result {
        use share::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let hash_password = match hash(&user_update.newpassword, DEFAULT_COST) {
                    Ok(h) => h,
                    Err(_) => panic!()
                };
        diesel::update(users)
            .filter(&id.eq(&user_update.user_id))
            .set((
                username.eq(user_update.newname),
                email.eq(user_update.newmail),
                password.eq(hash_password),
            )).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs{
                status: 200,
                message : "update  loginuser success.".to_string(),
        })
    }
}