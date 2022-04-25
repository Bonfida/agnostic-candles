use crate::{
    candles::candles::Candle,
    db::db::Database,
    error::WorkerError,
    utils::{markets::PythFeed, math::to_f64},
    PythContext,
};
use {
    bonfida_utils::pyth::get_oracle_price_fp32,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    tokio::{time, time::Duration},
};

pub async fn run_fetch_indexes(context: PythContext) {
    let mut interval = time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        if let Err(e) = fetch_indexes(&context.pyth_feeds, &context.db, &context.rpc).await {
            println!("Fetch index error {}", e)
        }
    }
}

pub async fn fetch_indexes(
    markets: &[PythFeed],
    database: &Database,
    rpc: &str,
) -> Result<(), WorkerError> {
    let connection = RpcClient::new(rpc.to_owned());

    let raw_accounts = connection
        .get_multiple_accounts(&markets.iter().map(|x| x.address).collect::<Vec<Pubkey>>())
        .await
        .map_err(|_| WorkerError::RpcError)?;

    for i in 0..raw_accounts.len() {
        let account = raw_accounts[i]
            .clone()
            .ok_or(WorkerError::AccountNotFound)?;

        let base_decimals = markets[i].base_decimals;
        let quote_decimals = markets[i].quote_decimals;

        let price = get_oracle_price_fp32(&account.data[..], base_decimals, quote_decimals);

        if let Ok(price) = price {
            let candle = Candle::new(to_f64(price));
            database
                .commit_candle(&candle, &markets[i].address.to_string(), &markets[i].name)
                .await
                .map_err(|_| WorkerError::DbCommitError)?
        } else {
            // Status `Not trading`
            continue;
        }
    }

    Ok(())
}
