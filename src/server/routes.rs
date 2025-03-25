use warp::{self, Filter};

use super::AppState;

pub mod handlers;

pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api = warp::path("api");

    let ping = api.and(warp::path("ping")).map(handlers::ping);

    // robot routes
    let start = api
        .and(warp::path("start"))
        .and(warp::post())
        // .and(warp::body::json())
        .and(warp::path::param())
        // .and(warp::header("Authorization"))
        .and(with_state(state.clone()))
        .and_then(handlers::robot::start);
    let stop = api
        .and(warp::path("stop"))
        .and(warp::post())
        // .and(warp::body::json())
        .and(warp::path::param())
        // .and(warp::header("Authorization"))
        .and(with_state(state.clone()))
        .and_then(handlers::robot::stop);
    let stop_buy = api
        .and(warp::path("stop"))
        .and(warp::post())
        // .and(warp::body::json())
        .and(warp::path::param())
        // .and(warp::header("Authorization"))
        .and(with_state(state.clone()))
        .and_then(handlers::robot::stop_buy);

    let reload_config = api
        .and(warp::path("stop"))
        .and(warp::post())
        // .and(warp::body::json())
        .and(warp::path::param())
        // .and(warp::header("Authorization"))
        .and(with_state(state.clone()))
        .and_then(handlers::robot::reload_config);

    warp::path::end()
        .map(handlers::index)
        .or(ping)
        .or(start)
        .or(stop)
        .or(stop_buy)
        .or(reload_config)
}

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
