// use diesel::{SqliteConnection, r2d2};
// use once_cell::sync::OnceCell;
// use std::sync::Arc;

// static CONNECTION_POOL: OnceCell<Arc<r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>>> =
//     OnceCell::new();

// /// Creates a new SQLite connection pool.
// pub fn new_connection_pool(
//     db_url: String,
// ) -> r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>> {
//     let manager = r2d2::ConnectionManager::<SqliteConnection>::new(db_url);
//     r2d2::Pool::builder()
//         .max_size(15)
//         .build(manager)
//         .unwrap_or_else(|_| panic!("Failed to create pool"))
// }

// /// Initializes the global connection pool if not already initialized.
// pub fn init_connection_pool(db_url: String) {
//     CONNECTION_POOL.get_or_init(|| {
//         Arc::new(new_connection_pool(db_url)) // Wrap the pool in `Arc` for thread safety
//     });
// }

// /// Retrieves the global connection pool.
// pub fn get_connection_pool() -> Arc<r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>> {
//     CONNECTION_POOL
//         .get()
//         .expect("Connection pool not initialized yet.")
//         .clone() // Return a clone of the Arc, allowing multiple threads to access the pool.
// }

// /// Get a connection from the pool.
// pub fn get_connection() -> Option<r2d2::PooledConnection<r2d2::ConnectionManager<SqliteConnection>>>
// {
//     let pool = get_connection_pool();
//     pool.get().ok() // Attempt to get a connection from the pool. Returns Option<PooledConnection>
// }

// // fn main() {

// //     // Initialize the pool once
// //     init_connection_pool();

// //     // Get a connection from the pool
// //     if let Some(conn) = get_connection() {
// //         println!("Successfully got a connection from the pool!");
// //         // Use `conn` here for database operations
// //     } else {
// //         println!("Failed to get a connection.");
// //     }
// // }
