use crate::{error::ServerError, structs::tradingview::SymbolInfo, Context};
use {
    actix_web::{get, web, HttpResponse},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct Params {
    pub symbol: String,
}

#[get("/symbols")]
pub async fn get_symbols_info(
    info: web::Query<Params>,
    context: web::Data<Context>,
) -> Result<HttpResponse, ServerError> {
    let symbol_info = SymbolInfo::new(&info.symbol, &context.markets)?;

    Ok(HttpResponse::Ok().json(symbol_info))
}
