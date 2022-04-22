use {
    chrono::serde::ts_seconds,
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    tokio_pg_mapper_derive::PostgresMapper,
};

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "candles")]
pub struct Candle {
    #[serde(with = "ts_seconds")]
    pub ts_start: DateTime<Utc>,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
}
