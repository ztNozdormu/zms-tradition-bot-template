use warp::{self, Filter};

use super::AppState;


pub mod handlers;

pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let _ = state;
    warp::path::end().map(handlers::index::index)
}

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
