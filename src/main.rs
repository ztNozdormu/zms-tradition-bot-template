use server::app;

mod server;

#[tokio::main]
async fn main() -> () {
    app::start().await;
}
