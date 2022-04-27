mod aob;
mod candles;
mod db;
mod error;
mod pyth;
mod utils;

use crate::{db::db::Database, utils::markets::load_markets};
use {
    clap::{Arg, Command},
    std::sync::Arc,
};

pub struct AobContext {
    pub db: Arc<Database>,
    pub rpc: String,
    pub aob_markets: Vec<utils::markets::AobMarket>,
}

pub struct PythContext {
    pub db: Arc<Database>,
    pub rpc: String,
    pub pyth_feeds: Vec<utils::markets::PythFeed>,
}

const MAX_ACCOUNT_CHUNK: usize = 100;

#[tokio::main]

async fn main() {
    let command = Command::new("candles-worker")
        .arg(Arg::new("markets_json_path").required(true))
        .arg(Arg::new("rpc").required(true))
        .arg(Arg::new("refresh_period").required(true));

    let matches = command.get_matches();

    let markets_path = matches.value_of("markets_json_path").unwrap();

    let rpc = matches.value_of("rpc").unwrap().to_owned();

    let refresh_period = matches
        .value_of("refresh_period")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let (pyth_feeds, aob_markets) = load_markets(markets_path, &rpc).await;

    let database = Arc::new(
        Database::new(
            refresh_period,
            (pyth_feeds.len() + aob_markets.len()) as u64,
        )
        .await
        .unwrap(),
    );

    let mut handles = vec![];
    for pyth_feed in pyth_feeds.chunks(MAX_ACCOUNT_CHUNK).map(|x| x.to_owned()) {
        let pyth_context = PythContext {
            db: database.clone(),
            rpc: rpc.clone(),
            pyth_feeds: pyth_feed,
        };
        handles.push(tokio::spawn(async move {
            pyth::fetch::run_fetch_indexes(pyth_context).await;
        }));
    }

    for aob_market in aob_markets.chunks(MAX_ACCOUNT_CHUNK).map(|x| x.to_owned()) {
        let aob_context = AobContext {
            db: database.clone(),
            rpc: rpc.clone(),
            aob_markets: aob_market,
        };
        handles.push(tokio::spawn(async move {
            aob::fetch::run_fetch_bbo(aob_context).await;
        }));
    }

    futures::future::join_all(handles).await;
}
