use crate::{
    error::ServerError,
    structs::{
        candles::Candle,
        context::Context,
        tradingview::{Bar, Resolution},
    },
    utils::{db::make_query, markets::valid_market, parse_query::parse_query, time::to_timestampz},
};
use {
    actix_web::{get, web, HttpRequest, HttpResponse},
    serde::Deserialize,
    tokio_pg_mapper::FromTokioPostgresRow,
};

#[derive(Debug, Deserialize)]
pub struct Params {
    pub symbol: String,
    pub from: u64,
    pub to: u64,
    pub resolution: String,
}

#[get("/history")]
pub async fn get_history(
    req: HttpRequest,
    context: web::Data<Context>,
) -> Result<HttpResponse, ServerError> {
    let params = parse_query::<Params>(&req)?;

    let resolution = Resolution::from_str(params.resolution.as_str())
        .map_err(|_| ServerError::WrongResolution)?;

    if !valid_market(&params.symbol, &context.markets.clone()) {
        return Err(ServerError::WrongParameters);
    }

    let conn = context
        .pool
        .get()
        .await
        .map_err(|_| ServerError::DbPoolError)?;

    let from_ts = to_timestampz(params.from);
    let to_ts = to_timestampz(params.to);

    let query = make_query(resolution);

    let candles = conn
        .query(&query, &[&from_ts, &to_ts, &params.symbol])
        .await
        .map_err(|_| ServerError::DbQuerryError)?
        .iter()
        .map(|row| Candle::from_row_ref(row).unwrap())
        .collect::<Vec<Candle>>();

    Ok(HttpResponse::Ok().json(Bar::candles_to_bar(candles)))
}
