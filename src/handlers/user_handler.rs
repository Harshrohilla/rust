use crate::models::rust_user::RustUser;
use crate::services::user_service;
use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn create_user(
    pool: web::Data<DbPool>,
    new_rust_user: web::Json<RustUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match user_service::create_rust_user(&mut conn, new_rust_user.into_inner()) {
        Ok(new_rust_user) => HttpResponse::Ok().json(new_rust_user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match user_service::get_users(&mut conn) {
        Ok(rust_users) => HttpResponse::Ok().json(rust_users),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match user_service::get_user(&mut conn, user_id.into_inner()) {
        Ok(rust_user) => HttpResponse::Ok().json(rust_user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match user_service::delete_user(&mut conn, user_id.into_inner()) {
        Ok(rust_user) => HttpResponse::Ok().json(rust_user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_user(pool: web::Data<DbPool>, updated_user: web::Json<RustUser>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match user_service::update_user(&mut conn, updated_user.into_inner()) {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
