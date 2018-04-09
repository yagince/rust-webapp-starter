use actix::*;
use actix_web::*;
use utils::schema::article;
use chrono::NaiveDateTime;
use model::response::{ArticleListMsgs, ArticleMsgs, Msgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub category: String,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="article"]
pub struct NewArticle<'a> {
    pub user_id: i32,
    pub category: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ArticleNew {
    pub user_id: i32,
    pub category: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ArticleId {
    pub article_id: i32,
}

pub struct ArticleList;

impl Message for ArticleList {
    type Result = Result<ArticleListMsgs, Error>;
}

impl Message for ArticleId {
    type Result = Result<ArticleMsgs, Error>;
}

impl Message for ArticleNew {
    type Result = Result<Msgs, Error>;
}
