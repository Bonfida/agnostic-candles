use actix_web::{web, Scope};

pub mod config;
pub mod history;
pub mod info;
pub mod pairs;
pub mod search;
pub mod time;

pub fn service() -> Scope {
    web::scope("/tradingview")
        .service(time::get_server_time)
        .service(config::get_tradingview_config)
        .service(info::get_symbols_info)
        .service(history::get_history)
        .service(pairs::get_pairs)
}
