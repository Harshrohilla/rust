use crate::models::rust_user::{NewUser, RustUser};
use crate::repositories::user_repository;
use diesel::PgConnection;

pub fn create_rust_user(
    conn: &mut PgConnection,
    new_rust_user: NewUser,
) -> Result<RustUser, diesel::result::Error> {
    user_repository::create_rust_user(conn, new_rust_user)
}

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<RustUser>, diesel::result::Error> {
    user_repository::get_users(conn)
}

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> Result<RustUser, diesel::result::Error> {
    println!("{}", user_id);

    user_repository::get_user(conn, user_id)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<usize, diesel::result::Error> {
    user_repository::delete_user(conn, user_id)
}

pub fn update_user(
    conn: &mut PgConnection,
    updated_user: RustUser,
) -> Result<RustUser, diesel::result::Error> {
    user_repository::update_user(conn, updated_user)
}
