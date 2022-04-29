use crate::structs::raw_market::RawMarket;
use std::fs::File;

pub fn load_markets(path: &str) -> Vec<RawMarket> {
    let reader = File::open(path).unwrap();
    serde_json::from_reader(reader).unwrap()
}

pub fn valid_market(market_name: &str, markets: &[RawMarket]) -> bool {
    markets.iter().any(|x| x.name == market_name)
}
