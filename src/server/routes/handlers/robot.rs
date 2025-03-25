mod responses;
use crate::server::{AppState, response::CusResponse};
use responses::RobotResponse;

// start the robot
pub async fn start(_token: String, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = RobotResponse::from(());

    Ok(warp::reply::json(&response))
}

// stop the robot
pub async fn stop(_token: String, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = RobotResponse::from(());

    Ok(warp::reply::json(&response))
}

// control the robot stop buy
pub async fn stop_buy(_token: String, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = RobotResponse::from(());

    Ok(warp::reply::json(&response))
}

// control the robot reload config
pub async fn reload_config(_token: String, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = RobotResponse::from(());

    Ok(warp::reply::json(&response))
}
