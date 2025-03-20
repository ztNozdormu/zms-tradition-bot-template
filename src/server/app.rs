use std::env;
use std::net::SocketAddr;
use warp::{self, Filter};

use crate::config;
use crate::server::routes;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct AppState {
    #[warn(dead_code)]
    pub jwt_secret: String,
}

pub async fn start() {
    // 清理环境变量
    let config = config::load_config().expect("config must be set");

    let bind_address: SocketAddr = config.server.address.parse().expect("地址解析失败");

    let jwt_secret = config.server.jwt_secret;

    // TODO start bot
    // init app
    let app_state = AppState { jwt_secret };

    let routes = routes::routes(app_state).with(warp::log(APPLICATION_NAME));

    println!("You can access the server at {}", bind_address);

    warp::serve(routes).run(bind_address).await;
}
