use actix_web::actix::Addr;
use model::db::ConnDsl;

pub struct AppState {
    pub db: Addr<ConnDsl>,
}
