use crate::schema::rust_user;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = rust_user)]
pub struct RustUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
