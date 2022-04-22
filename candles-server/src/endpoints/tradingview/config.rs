use crate::structs::tradingview::{Config, RESOLUTIONS};
use actix_web::{get, web, Responder};

const CONFIG: Config = Config {
    supported_resolutions: RESOLUTIONS,
    supports_group_request: false,
    supports_marks: false,
    supports_search: true,
    supports_timescale_marks: false,
};

#[get("/config")]
pub async fn get_tradingview_config() -> impl Responder {
    web::Json(CONFIG)
}
