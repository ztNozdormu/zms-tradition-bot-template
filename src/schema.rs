// @generated automatically by Diesel CLI.

diesel::table! {
    KeyValueStore (id) {
        id -> Integer,
        key -> Text,
        value_type -> Text,
        string_value -> Nullable<Text>,
        datetime_value -> Nullable<Timestamp>,
        float_value -> Nullable<Float>,
        int_value -> Nullable<Integer>,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        ft_trade_id -> Integer,
        ft_order_side -> Text,
        ft_pair -> Text,
        ft_is_open -> Bool,
        ft_amount -> Float,
        ft_price -> Float,
        ft_cancel_reason -> Nullable<Text>,
        order_id -> Text,
        status -> Nullable<Text>,
        symbol -> Nullable<Text>,
        order_type -> Nullable<Text>,
        side -> Nullable<Text>,
        price -> Nullable<Float>,
        average -> Nullable<Float>,
        amount -> Nullable<Float>,
        filled -> Nullable<Float>,
        remaining -> Nullable<Float>,
        cost -> Nullable<Float>,
        stop_price -> Nullable<Float>,
        order_date -> Nullable<Timestamp>,
        order_filled_date -> Nullable<Timestamp>,
        order_update_date -> Nullable<Timestamp>,
        funding_fee -> Nullable<Float>,
        ft_fee_base -> Nullable<Float>,
        ft_order_tag -> Nullable<Text>,
    }
}

diesel::table! {
    pairlocks (id) {
        id -> Integer,
        pair -> Text,
        side -> Text,
        reason -> Nullable<Text>,
        lock_time -> Timestamp,
        lock_end_time -> Timestamp,
        active -> Bool,
    }
}

diesel::table! {
    trade_custom_data (id) {
        id -> Integer,
        ft_trade_id -> Nullable<Integer>,
        cd_key -> Text,
        cd_type -> Text,
        cd_value -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    trades (id) {
        id -> Integer,
        exchange -> Text,
        pair -> Text,
        base_currency -> Nullable<Text>,
        stake_currency -> Nullable<Text>,
        is_open -> Bool,
        fee_open -> Float,
        fee_open_cost -> Nullable<Float>,
        fee_open_currency -> Nullable<Text>,
        fee_close -> Float,
        fee_close_cost -> Nullable<Float>,
        fee_close_currency -> Nullable<Text>,
        open_rate -> Float,
        open_rate_requested -> Nullable<Float>,
        open_trade_value -> Nullable<Float>,
        close_rate -> Nullable<Float>,
        close_rate_requested -> Nullable<Float>,
        realized_profit -> Nullable<Float>,
        close_profit -> Nullable<Float>,
        close_profit_abs -> Nullable<Float>,
        stake_amount -> Float,
        max_stake_amount -> Nullable<Float>,
        amount -> Float,
        amount_requested -> Nullable<Float>,
        open_date -> Timestamp,
        close_date -> Nullable<Timestamp>,
        stop_loss -> Nullable<Float>,
        stop_loss_pct -> Nullable<Float>,
        initial_stop_loss -> Nullable<Float>,
        initial_stop_loss_pct -> Nullable<Float>,
        is_stop_loss_trailing -> Bool,
        max_rate -> Nullable<Float>,
        min_rate -> Nullable<Float>,
        exit_reason -> Nullable<Text>,
        exit_order_status -> Nullable<Text>,
        strategy -> Nullable<Text>,
        enter_tag -> Nullable<Text>,
        timeframe -> Nullable<Integer>,
        trading_mode -> Nullable<Text>,
        amount_precision -> Nullable<Float>,
        price_precision -> Nullable<Float>,
        precision_mode -> Nullable<Integer>,
        precision_mode_price -> Nullable<Integer>,
        contract_size -> Nullable<Float>,
        leverage -> Nullable<Float>,
        is_short -> Bool,
        liquidation_price -> Nullable<Float>,
        interest_rate -> Float,
        funding_fees -> Nullable<Float>,
        funding_fee_running -> Nullable<Float>,
    }
}

diesel::joinable!(orders -> trades (ft_trade_id));
diesel::joinable!(trade_custom_data -> trades (ft_trade_id));

diesel::allow_tables_to_appear_in_same_query!(
    KeyValueStore,
    orders,
    pairlocks,
    trade_custom_data,
    trades,
);
