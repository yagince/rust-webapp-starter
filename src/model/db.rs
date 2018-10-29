use actix_web::actix::{Addr,SyncArbiter,Actor,SyncContext};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager };
use dotenv;
use num_cpus;

pub struct ConnDsl(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}

pub fn init() -> Addr<ConnDsl> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder().max_size(5).build(manager).expect("Failed to create pool.");
    SyncArbiter::start( num_cpus::get(), move || { ConnDsl(conn.clone()) })
}
