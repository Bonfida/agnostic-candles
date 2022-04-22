use crate::structs::raw_market::RawMarket;
use {bb8::Pool, bb8_postgres::PostgresConnectionManager, tokio_postgres::NoTls};

#[derive(Clone)]
pub struct Context {
    pub markets: Vec<RawMarket>,
    pub pool: Pool<PostgresConnectionManager<NoTls>>,
}
