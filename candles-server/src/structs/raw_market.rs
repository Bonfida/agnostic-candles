use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMarket {
    pub name: String,
    pub address: String,
    pub is_pyth: bool,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub min_mov: f64,
    pub price_scale: f64,
}
