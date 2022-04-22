mod aob;
mod candles;
mod db;
mod error;
mod pyth;
mod utils;

use crate::{db::db::Database, utils::markets::load_markets};
use {
    clap::{Arg, Command},
    tokio::{time, time::Duration},
};

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

    let database = Database::new(
        refresh_period,
        (pyth_feeds.len() + aob_markets.len()) as u64,
    )
    .await
    .unwrap();

    let mut interval = time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        if let Err(e) = aob::fetch::fetch_bbo(&aob_markets, &database, &rpc).await {
            println!("Fetch bbo error {}", e)
        };
        if let Err(e) = pyth::fetch::fetch_indexes(&pyth_feeds, &database, &rpc).await {
            println!("Fetch pyth error {}", e)
        }
    }
}
