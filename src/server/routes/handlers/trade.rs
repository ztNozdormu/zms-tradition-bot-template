use crate::server::AppState;
use crate::server::response::CusResponse;

mod responses;


// get trades list recently per req limit 500 count trade data
pub async fn trades( _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = crate::server::routes::handlers::trade::responses::TradeResponse::from(());

    Ok(warp::reply::json(&response))
}

// get trade by id
pub async fn get_trade_by_id(trade_id: u32, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = crate::server::routes::handlers::trade::responses::TradeResponse::from(());

    Ok(warp::reply::json(&response))
}

// delete trade by trade_id
pub async fn delete_trade_by_id(trade_id: u32, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = crate::server::routes::handlers::trade::responses::TradeResponse::from(());

    Ok(warp::reply::json(&response))
}

// cancel unopen trade by trade_id
pub async fn cancel_trade_by_id(trade_id: u32, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = crate::server::routes::handlers::trade::responses::TradeResponse::from(());

    Ok(warp::reply::json(&response))
}

// reload trade from exchange by trade_id
pub async fn reload_trade_from_exchange(trade_id: u32, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = crate::server::routes::handlers::trade::responses::TradeResponse::from(());

    Ok(warp::reply::json(&response))
}

// list all uncompleted trades
pub async fn status(trade_id: u32, _state: AppState) -> CusResponse {
    // TODO
    // valid token
    // let _user_id = decode_token(&state.jwt_secret, &token).unwrap().user_id();

    let response = crate::server::routes::handlers::trade::responses::TradeResponse::from(());

    Ok(warp::reply::json(&response))
}

