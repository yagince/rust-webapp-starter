use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use futures::future::Future;
use chrono::Utc;

use handler::index::State;
use model::article::{Article, ArticleList, ArticleId, NewArticle, ArticleNew};
use model::db::ConnDsl;
use model::response::{ArticleListMsgs, ArticleMsgs, Msgs};


pub fn article(req: HttpRequest<State>) -> Result<Box<Future<Item=HttpResponse, Error=Error>>, Error>{
    let header_article_id = req.match_info().get("article_id").unwrap();
    let article_id: i32 = header_article_id.parse().map_err(error::ErrorBadRequest)?;
    Ok(req.state().db.send(ArticleId{article_id})
       .from_err()
       .and_then(|res| {
           match res {
               Ok(article) =>
                   Ok(httpcodes::HTTPOk.build().json(article)?),
               Err(_) =>
                   Ok(httpcodes::HTTPInternalServerError.into()),
           }
       }).responder())
}

impl Handler<ArticleId> for ConnDsl {
    type Result = Result<ArticleMsgs, Error>;

    fn handle(&mut self, article_id: ArticleId, _: &mut Self::Context) -> Self::Result {
        use utils::schema::article::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let the_article =  article.filter(&id.eq(&article_id.article_id)).load::<Article>(conn).map_err(error::ErrorInternalServerError)?.pop();
        match the_article {
            Some(the_article) => {
                    let current_article = Article {
                        id: the_article.id,
                        user_id: the_article.user_id,
                        category: the_article.category.clone(),
                        title: the_article.title.clone(),
                        body: the_article.body.clone(),
                        created_at: the_article.created_at.clone(),
                    };
                    Ok(ArticleMsgs { 
                        status: 200,
                        message : "The  current_user info.".to_string(),
                        article: current_article,
                    })
            },
            None => {
                    let no_article = Article {
                        id: 0,
                        user_id: 0,
                        category: "".to_owned(),
                        title: "".to_owned(),
                        body: "".to_owned(),
                        created_at: Utc::now().naive_utc(),
                    };
                    Ok(ArticleMsgs { 
                        status: 400,
                        message : "error.".to_string(),
                        article: no_article,
                    })
            },
        }
    }
}

pub fn article_list(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.state().db.send(ArticleList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(article_list) =>
                    Ok(httpcodes::HTTPOk.build().json(article_list)?),
                Err(_) =>
                    Ok(httpcodes::HTTPInternalServerError.into()),
            }
        }).responder()
}

impl Handler<ArticleList> for ConnDsl {
    type Result = Result<ArticleListMsgs, Error>;

    fn handle(&mut self, article_list: ArticleList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::article::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let articles = article.load::<Article>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(ArticleListMsgs { 
            status: 200,
            message : "article_list result.".to_string(),
            article_list: articles,
        })
    }
}

pub fn article_new(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.clone().json()                     
       .from_err()
       .and_then(move |article_new: ArticleNew| {  
            req.state().db.send(ArticleNew{ 
                user_id: article_new.user_id,
                category: article_new.category,
                title: article_new.title,
                content: article_new.content,
            })         
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(msg) => Ok(httpcodes::HTTPOk.build().json(msg)?),
                    Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                }
            })
        }).responder()
}

impl Handler<ArticleNew> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, article_new: ArticleNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::article::dsl::*;

        let new_article = NewArticle {
            user_id: article_new.user_id,
            category: &article_new.category,
            title: &article_new.title,
            body: &article_new.content,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::insert_into(article).values(&new_article).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                    status: 200,
                    message : "Article Publish Successful.".to_string(),
        })        
    }
}
