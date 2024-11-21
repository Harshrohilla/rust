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
use handlers::user_handler::{create_user, delete_user, get_user, get_users, update_user};

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
            .route("/users/create", web::post().to(create_user))
            .route("/users/get/all", web::get().to(get_users))
            .route("/users/get/{id}", web::get().to(get_user))
            .route("/users/delete/{id}", web::delete().to(delete_user))
            .route("/users/update", web::put().to(update_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
