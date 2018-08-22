use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use share::state::AppState;
use model::article::{ArticleList, ArticleId, ArticleNew};


pub fn article(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let header_article_id = req.match_info().get("article_id").unwrap();
    let article_id: i32 = header_article_id.parse().unwrap();
    req.state().db.send(ArticleId{article_id})
       .from_err()
       .and_then(|res| {
           match res {
               Ok(article) =>
                   Ok(HttpResponse::Ok().json(article)),
               Err(_) =>
                   Ok(HttpResponse::InternalServerError().into()),
           }
       }).responder()
}

pub fn article_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
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

pub fn article_new((article_new, state): (Json<ArticleNew>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(ArticleNew{ 
            user_id: article_new.user_id.clone(),
            category: article_new.category.clone(),
            title: article_new.title.clone(),
            content: article_new.content.clone(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

