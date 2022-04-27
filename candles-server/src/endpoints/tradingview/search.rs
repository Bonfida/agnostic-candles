use crate::{
    error::ServerError,
    structs::{context::Context, tradingview::Symbol},
};
use {
    actix_web::{get, web, HttpResponse},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct Params {
    pub query: String,
    pub search_type: String,
    pub exchange: String,
    pub limit: u8,
}

#[get("/search")]
pub async fn get_search(
    context: web::Data<Context>,
    info: web::Query<Params>,
) -> Result<HttpResponse, ServerError> {
    if info.search_type.as_str() != "spot" {
        return Err(ServerError::WrongParameters);
    }
    if info.exchange.as_str() != "Bonfida" {
        return Err(ServerError::WrongParameters);
    }

    let markets = context
        .into_inner()
        .markets
        .clone()
        .into_iter()
        .filter(|x| {
            // TODO change for a better search
            x.name.contains(info.query.as_str())
                || info.query.contains(x.name.as_str())
                || x.address.contains(info.query.as_str())
        })
        .map(|x| Symbol {
            // TODO maybe add decimals?
            description: format!("Name:{} - Address: {}", x.name, x.address),
            full_name: x.name.clone(),
            symbol: x.name.clone(),
            exchange: "Bonfida".to_owned(),
            ticker: x.name,
            symbol_type: "Spot".to_owned(),
        })
        .collect::<Vec<Symbol>>();

    Ok(HttpResponse::Ok().json(markets))
}
