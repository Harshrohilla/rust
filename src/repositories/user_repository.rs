use crate::models::rust_user::{NewUser, RustUser};
use crate::schema::rust_user::dsl::*;
use diesel::prelude::*;

pub fn create_rust_user(conn: &mut PgConnection, new_rust_user: NewUser) -> QueryResult<RustUser> {
    diesel::insert_into(rust_user)
        .values(&new_rust_user)
        .get_result(conn)
}

pub fn get_users(conn: &mut PgConnection) -> QueryResult<Vec<RustUser>> {
    rust_user.load::<RustUser>(conn)
}

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<RustUser> {
    rust_user.find(user_id).get_result(conn)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<usize> {
    diesel::delete(rust_user.find(user_id)).execute(conn)
}

pub fn update_user(conn: &mut PgConnection, updated_user: RustUser) -> QueryResult<RustUser> {
    diesel::update(rust_user.find(updated_user.id))
        .set(&updated_user)
        .get_result(conn)
}
