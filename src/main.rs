pub mod config;
pub mod schema;
pub mod models {
    pub mod rust_user;
}
pub mod repositories {
    pub mod user_repository;
}
pub mod services {
    pub mod user_service;
}
pub mod handlers {
    pub mod user_handler;
}

use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use handlers::user_handler::{self};

// type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_handler::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
