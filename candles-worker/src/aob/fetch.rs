use crate::{
    candles::candles::Candle,
    db::db::Database,
    error::WorkerError,
    utils::{markets::AobMarket, math::to_f64},
    AobContext,
};
use {
    agnostic_orderbook::critbit::Slab,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    tokio::{time, time::Duration},
};

pub async fn run_fetch_bbo(context: AobContext) {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        if let Err(e) = fetch_bbo(&context.aob_markets, &context.db, &context.rpc).await {
            println!("Fetch bbo error {}", e)
        }
    }
}

pub async fn fetch_bbo(
    markets: &[AobMarket],
    database: &Database,
    rpc: &str,
) -> Result<(), WorkerError> {
    let connection = RpcClient::new(rpc.to_owned());
    let raw_bids = connection
        .get_multiple_accounts(
            &markets
                .iter()
                .map(|m| m.bids_address)
                .collect::<Vec<Pubkey>>(),
        )
        .await
        .map_err(|_| WorkerError::RpcError)?;

    let raw_asks = connection
        .get_multiple_accounts(
            &markets
                .iter()
                .map(|m| m.asks_address)
                .collect::<Vec<Pubkey>>(),
        )
        .await
        .map_err(|_| WorkerError::RpcError)?;

    for i in 0..raw_asks.len() {
        let bbo = {
            // Scoped to prevent `Send` trait error
            let mut ask_account = raw_asks[i].clone().ok_or(WorkerError::AccountNotFound)?;
            let mut bid_account = raw_bids[i].clone().ok_or(WorkerError::AccountNotFound)?;

            let ask_slab = Slab::from_bytes(
                &mut ask_account.data[..],
                markets[i].callback_info_len as usize,
            );
            let bid_slab = Slab::from_bytes(
                &mut bid_account.data[..],
                markets[i].callback_info_len as usize,
            );
            let best_ask = ask_slab
                .find_min()
                .map(|h| ask_slab.get_node(h).unwrap().as_leaf().unwrap().price());

            let best_bid = bid_slab
                .find_min()
                .map(|h| bid_slab.get_node(h).unwrap().as_leaf().unwrap().price());

            match (best_bid, best_ask) {
                (Some(best_bid), Some(best_ask)) => (best_bid + best_ask) / 2,
                (Some(x), None) | (None, Some(x)) => x,
                (None, None) => 0,
            }
        };

        let candle = Candle::new(to_f64(bbo));
        database
            .commit_candle(&candle, &markets[i].address.to_string(), &markets[i].name)
            .await
            .map_err(|_| WorkerError::DbCommitError)?;
    }

    Ok(())
}
