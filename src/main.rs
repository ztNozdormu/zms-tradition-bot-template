use server::app;

mod config;
mod server;
mod db;


#[tokio::main]
async fn main() -> () {
    app::start().await;
}
