use actix_web::{HttpMessage, HttpRequest, HttpResponse, error, Error, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::State;
use model::article::{ArticleList, ArticleId, ArticleNew};


pub fn article(req: HttpRequest<State>) -> Result<Box<Future<Item=HttpResponse, Error=Error>>, Error>{
    let header_article_id = req.match_info().get("article_id").unwrap();
    let article_id: i32 = header_article_id.parse().map_err(error::ErrorBadRequest)?;
    Ok(req.state().db.send(ArticleId{article_id})
       .from_err()
       .and_then(|res| {
           match res {
               Ok(article) =>
                   Ok(HttpResponse::Ok().json(article)),
               Err(_) =>
                   Ok(HttpResponse::InternalServerError().into()),
           }
       }).responder())
}

pub fn article_list(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.state().db.send(ArticleList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(article_list) =>
                    Ok(HttpResponse::Ok().json(article_list)),
                Err(_) =>
                    Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
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
                    Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                    Err(_) => Ok(HttpResponse::InternalServerError().into())
                }
            })
        }).responder()
}

