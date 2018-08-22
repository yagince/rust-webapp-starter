use actix_web::{fs::NamedFile, HttpRequest, Error, Result};

pub fn index(_req: &HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}

pub fn path(_req: &HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html")?)
}