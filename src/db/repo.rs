use std::env;

use diesel::{r2d2, SqliteConnection};

pub fn new_connection_pool() -> r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>> {

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.");
    pool
}