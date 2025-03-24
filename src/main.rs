mod config;
mod db;
mod server;

#[tokio::main]
async fn main() -> () {
    server::start().await;
}
