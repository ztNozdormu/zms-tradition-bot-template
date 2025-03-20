use std::env;
use std::net::SocketAddr;
use warp::{self, Filter};

use crate::server::routes;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct AppState {
    // TODO
}

pub async fn start() {
    // 清理环境变量
    let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let _jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // TODO start bot
    // init app
    let app_state = AppState {};

    let routes = routes::routes(app_state).with(warp::log(APPLICATION_NAME));

    println!("You can access the server at {}", bind_address);

    warp::serve(routes).run(bind_address).await;
}
