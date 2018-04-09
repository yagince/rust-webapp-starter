use actix::*;
use actix_web::*;
use utils::schema::users;
use chrono::NaiveDateTime;
use model::response::{Msgs, SigninMsgs, UserInfoMsgs};

#[derive(Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize,Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct SignupUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct SigninUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
}

impl Message for SignupUser {
    type Result = Result<Msgs, Error>;
}

impl Message for SigninUser {
    type Result = Result<SigninMsgs, Error>;
}

impl Message for UserInfo {
    type Result = Result<UserInfoMsgs, Error>;
}