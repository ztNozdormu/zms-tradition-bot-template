use diesel::sqlite::SqliteConnection;
use bb8::Pool;
use bb8_diesel::DieselConnectionManager;
use std::{env, sync::Mutex};
use once_cell::sync::Lazy;

type SqlitePool = Pool<DieselConnectionManager<SqliteConnection>>;

pub static GLOBAL_CONNECTION: Lazy<Mutex<SqlitePool>> = Lazy::new(|| Mutex::new(establish_connection()));

pub fn establish_connection() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url{:?}", database_url);
    // Create the Diesel connection manager
    let manager = DieselConnectionManager::<SqliteConnection>::new(database_url);
    // Create the connection pool with a max size of 10
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool")
}

