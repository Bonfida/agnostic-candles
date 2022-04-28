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
    let market = context
        .markets
        .clone()
        .into_iter()
        .find(|x| x.name == info.symbol)
        .ok_or(ServerError::SymbolNotFound)?;

    let symbol_info = SymbolInfo::new(&info.symbol, market)?;

    Ok(HttpResponse::Ok().json(symbol_info))
}
