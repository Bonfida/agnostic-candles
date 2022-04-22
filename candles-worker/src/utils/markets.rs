use {
    agnostic_orderbook::state::MarketState,
    bytemuck::from_bytes,
    serde::{Deserialize, Serialize},
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    std::fs::File,
    std::str::FromStr,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMarket {
    pub name: String,
    pub address: String,
    pub is_pyth: bool,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}

#[derive(Clone)]
pub struct AobMarket {
    pub name: String,
    pub address: Pubkey,
    pub bids_address: Pubkey,
    pub asks_address: Pubkey,
    pub callback_info_len: u64,
}

#[derive(Clone)]
pub struct PythFeed {
    pub name: String,
    pub address: Pubkey,
    pub base_decimals: u8,
    pub quote_decimals: u8,
}

pub async fn load_markets(path: &str, rpc: &str) -> (Vec<PythFeed>, Vec<AobMarket>) {
    let markets: Vec<RawMarket> = {
        let reader = File::open(path).unwrap();
        serde_json::from_reader(reader).unwrap()
    };

    // Create `PythFeed` list
    let pyth_feed = markets
        .iter()
        .filter(|m| m.is_pyth)
        .map(|m| PythFeed {
            name: m.name.to_owned(),
            address: Pubkey::from_str(&m.address).unwrap(),
            base_decimals: m.base_decimals,
            quote_decimals: m.quote_decimals,
        })
        .collect::<Vec<PythFeed>>();

    // Create `AobMarket` list
    let raw_aob_markets = markets
        .iter()
        .filter(|m| !m.is_pyth)
        .collect::<Vec<&RawMarket>>();

    let aob_markets = retrieve_aob_market(raw_aob_markets, rpc).await;

    (pyth_feed, aob_markets)
}

pub async fn retrieve_aob_market(raw_markets: Vec<&RawMarket>, rpc: &str) -> Vec<AobMarket> {
    let connection = RpcClient::new(rpc.to_owned());
    let pubkeys = raw_markets
        .clone()
        .into_iter()
        .map(|x| Pubkey::from_str(&x.address).unwrap())
        .collect::<Vec<Pubkey>>();
    let market_state_raws = connection.get_multiple_accounts(&pubkeys).await.unwrap();
    let market_states = market_state_raws
        .iter()
        .map(|ob| {
            from_bytes::<MarketState>(
                ob.as_ref()
                    .and_then(|a| a.data.get(0..agnostic_orderbook::state::MARKET_STATE_LEN))
                    .unwrap(),
            )
        })
        .collect::<Vec<&MarketState>>();
    let bid_keys = market_states
        .iter()
        .map(|m| Pubkey::new(&m.bids))
        .collect::<Vec<Pubkey>>();
    let ask_keys = market_states
        .iter()
        .map(|m| Pubkey::new(&m.asks))
        .collect::<Vec<Pubkey>>();

    let mut aob_markets: Vec<AobMarket> = Vec::new();

    for i in 0..market_states.len() {
        aob_markets.push({
            AobMarket {
                name: raw_markets[i].name.to_owned(),
                address: pubkeys[i],
                bids_address: bid_keys[i],
                asks_address: ask_keys[i],
                callback_info_len: market_states[i].callback_info_len,
            }
        })
    }

    aob_markets
}
