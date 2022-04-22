use crate::{
    error::ServerError, structs::tradingview::SymbolInfo, utils::parse_query::parse_query,
};
use {
    actix_web::{get, HttpRequest, HttpResponse},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct Params {
    pub symbol: String,
}

#[get("/symbols")]
pub async fn get_symbols_info(req: HttpRequest) -> Result<HttpResponse, ServerError> {
    let params = parse_query::<Params>(&req)?;

    let symbol_info = SymbolInfo::new(params.symbol);

    Ok(HttpResponse::Ok().json(symbol_info))
}
