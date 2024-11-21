use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
}