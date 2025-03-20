use server::app;

mod config;
mod server;

#[tokio::main]
async fn main() -> () {
    app::start().await;
}
