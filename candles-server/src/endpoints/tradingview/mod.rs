use actix_web::{web, Scope};

pub mod config;
pub mod history;
pub mod pairs;
pub mod search;
pub mod symbols;
pub mod time;

pub fn service() -> Scope {
    web::scope("/tradingview")
        .service(time::get_server_time)
        .service(config::get_tradingview_config)
        .service(symbols::get_symbols_info)
        .service(history::get_history)
        .service(search::get_search)
        .service(pairs::get_pairs)
}
