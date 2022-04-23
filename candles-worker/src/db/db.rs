use {
    std::time::Duration,
    sysinfo::SystemExt,
    tokio_postgres::{tls::MakeTlsConnect, types::Type, NoTls, Socket, Statement},
};

use chrono::{NaiveDateTime, Utc};

use crate::candles::candles::Candle;

pub struct Database {
    client: tokio_postgres::Client,
    insertion_statement: Statement,
}

impl Database {
    pub const ENTRY_SIZE: u64 = 112; // Size in bytes of a single db entry
    pub const RELATIVE_CHUNK_SIZE: f64 = 0.10; // Size of a timescaledb chunk
    pub async fn new(
        refresh_period_ms: u64,
        number_of_markets: u64,
    ) -> Result<Self, tokio_postgres::Error> {
        let (client, connection) = connect_to_database().await;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        initialize(&client, refresh_period_ms, number_of_markets).await?;
        let insertion_statement = client
            .prepare(
                "INSERT INTO candles VALUES ($1, $2, $3, $4, $5, $6, $7) 
            ON CONFLICT (address, timestamp) DO UPDATE
            SET close = EXCLUDED.close,
                low = LEAST(EXCLUDED.low, candles.low),
                high = GREATEST(EXCLUDED.high, candles.high);",
            )
            .await
            .unwrap();
        Ok(Self {
            client,
            insertion_statement,
        })
    }

    pub async fn commit_candle(
        &self,
        candle: &Candle,
        address: &String,
        name: &String,
    ) -> Result<(), tokio_postgres::Error> {
        self.client
            .execute(
                &self.insertion_statement,
                &[
                    &chrono::DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(candle.ts_start, 0),
                        Utc,
                    ),
                    address,
                    name,
                    &candle.open,
                    &candle.close,
                    &candle.high,
                    &candle.low,
                ],
            )
            .await?;
        Ok(())
    }
}

async fn connect_to_database() -> (
    tokio_postgres::Client,
    tokio_postgres::Connection<Socket, <tokio_postgres::NoTls as MakeTlsConnect<Socket>>::Stream>,
) {
    let password = std::env::var("POSTGRES_PASSWORD")
        .expect("POSTGRES_PASSWORD environment variable must be set!");
    let config_str = format!("host=db port=5432 password={password} user=postgres dbname=postgres");
    loop {
        let res = tokio_postgres::connect(&config_str, NoTls).await;
        if let Ok(r) = res {
            return r;
        }
        println!("Failed to connect to database, retrying");
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

async fn initialize(
    client: &tokio_postgres::Client,
    refresh_period_ms: u64,
    mut number_of_markets: u64,
) -> Result<(), tokio_postgres::Error> {
    number_of_markets = std::cmp::max(10, number_of_markets);
    println!("=== Initializing database ===");
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS candles (
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        address VARCHAR(44), 
        name VARCHAR(20),
        open DOUBLE PRECISION,
        close DOUBLE PRECISION,
        high DOUBLE PRECISION,
        low DOUBLE PRECISION,
        PRIMARY KEY (timestamp, address)
    );",
            &[],
        )
        .await
        .unwrap();
    // We convert the table to a hypertable
    let o = client
        .query(
            "SELECT create_hypertable('candles', 'timestamp', if_not_exists => TRUE);",
            &[],
        )
        .await
        .unwrap();
    println!("Output from create_hypertable");
    println!("{o:?}");

    // Implements the best practice detailed here
    // https://docs.timescale.com/timescaledb/latest/how-to-guides/hypertables/best-practices/#time-intervals
    let system_memory_kb = sysinfo::System::new_all().total_memory();
    let chunk_size_ms =
        refresh_period_ms * system_memory_kb * 1024 / Database::ENTRY_SIZE / number_of_markets;
    let chunk_size_ms = (chunk_size_ms as f64) * Database::RELATIVE_CHUNK_SIZE;
    let s = client
        .prepare_typed(
            "SELECT set_chunk_time_interval('candles', $1);",
            &[Type::INT8],
        )
        .await
        .unwrap();
    let o = client.query(&s, &[&(chunk_size_ms as i64)]).await?;
    println!("Output from set_chunk_time_interval");
    println!("{o:?}");
    Ok(())
}
