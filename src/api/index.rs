use actix::*;
use actix_web::{fs::NamedFile, HttpRequest, Result};
use std::path::Path;
use model::db::ConnDsl;

pub struct State {
    pub db: Addr<Syn, ConnDsl>
}

pub fn home(_req: HttpRequest<State>) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("public/index.html"))?)
}

pub fn path(_req: HttpRequest<State>) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("public/index.html"))?)
}
