use actix_web::{Error,actix::Message};
use share::schema::users;
use chrono::{Utc, NaiveDateTime};
use model::response::{Msgs, SigninMsgs, UserInfoMsgs};
use model::response::MyError;

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

#[derive(Deserialize,Serialize, Debug)]
pub struct UserUpdate {
    pub user_id: i32,
    pub newname: String,
    pub newmail: String,
    pub newpassword: String,
    pub confirm_newpassword: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDelete {
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

impl Message for UserUpdate {
    type Result = Result<Msgs, Error>;
}
impl Message for UserDelete {
    type Result = Result<Msgs, MyError>;
}

impl User {
    pub fn new() -> User {
        User {
            id: 0,
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}