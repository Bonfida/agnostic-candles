<h1 align="center">Agnostic candles</h1>
<br />
<p align="center">
<img width="250" src="https://i.imgur.com/nn7LMNV.png"/>
</p>
<p align="center">
<a href="https://twitter.com/bonfida">
<img src="https://img.shields.io/twitter/url?label=Bonfida&style=social&url=https%3A%2F%2Ftwitter.com%2Fbonfida">
</a>
</p>

<br />

<div align="center">
<img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" />
</div>

<br />
<h2 align="center">Table of contents</h2>
<br />

1. [Timescaledb](#timescaledb)
2. [Configuration](#configuration)
3. [Worker](#worker)
4. [Server](#server)
5. [Deployment](#deployment)

Tradingview documentation can be found [here](https://github.com/tradingview/charting_library/wiki/UDF) this is a private repository, you must ask Tradingview to get access to it.

<br />
<a name="timescaledb"></a>
<h2 align="center">Timescaledb</h2>
<br />

This repository uses TimescaleDB to store candles. Full documentation can be found on their [website](https://www.timescale.com/)

<br />
<a name="configuration"></a>
<h2 align="center">Configuration</h2>
<br />

Markets should be passed in a JSON file as follow

| address                         | name               | isPyth                     | baseDecimals                        | quoteDecimals                        | min_mov              | price_scale             |
| ------------------------------- | ------------------ | -------------------------- | ----------------------------------- | ------------------------------------ | -------------------- | ----------------------- |
| string                          | string             | bool                       | u8                                  | u8                                   | u8                   | u16                     |
| Pyth feed or AOB market address | Name of the market | `true` if this a Pyth feed | Base token decimals (for Pyth only) | Quote token decimals (for Pyth only) | Min move (cf TV doc) | Price scale (cf TV doc) |

`min_mov` and `price_scale` are parameters required by the Tradingview specification, see the exact definition on [Tradingview documentation](https://github.com/tradingview/charting_library/wiki/UDF)

- `min_mov = 1` is equivalent to a tick size of 0.01
- `price_scale = 100` is equivalent to 1.01

For instance

```json
[
  {
    "address": "ETp9eKXVv1dWwHSpsXRUuXHmw24PwRkttCGVgpZEY9zF",
    "name": "FIDA-USDC-PYTH",
    "isPyth": true,
    "baseDecimals": 6,
    "quoteDecimals": 6,
    "minMov": 1,
    "priceScale": 100
  }
]
```

<br />
<a name="worker"></a>
<h2 align="center">Worker</h2>
<br />

The worker directory contains the program that fetches price information from the blockchain and stores it in the database.

It takes the following parameters in input:

```
cargo run markets_json_path rpc refresh_period
```

- `markets_json_path` is the path to your JSON file that contains the markets you want to fetch
- `rpc` The URL of your Solana RPC endpoint
- `refresh_period` interval at which candles are fetched (in ms)

So for instance

```
cargo run ./markets.json https://solana-api.projectserum.com 10000
```

The worker handles Pyth and AOB markets in separate threads. This program uses `getMultipleAccountInfo` RPC requests to optimize the number of RPC calls. However, certain RPC nodes have limits to how many accounts can be passed in 1 request (usually 100) this is why the array of accounts are split in chunks of `MAX_ACCOUNT_CHUNK` and spawned in different threads.

For AOB markets, bids and asks account addresses are being fetched at the start of the program and are cached for more efficient polling.

<br />
<a name="server"></a>
<h2 align="center">Server</h2>
<br />

The server uses [actix web]() and is served by default on port `8080` .

It takes the following parameters in input:

```
cargo run markets_json_path user password host port dbname
```

For instance

```
cargo run ./market.json my_user my_password 127.0.0.1 5432 my_db
```

it has the following endpoints required by the Tradingview [specification](https://github.com/tradingview/charting_library/wiki/UDF)

### Config

**Request:**

`GET /tradingview/config`

It exposes the tradingview configuration of your server
If you want to change the available resolutions of your server you will have to modify this configuration

**Response:**

```json
{
  "supported_resolutions": [
    "1",
    "3",
    "5",
    "15",
    "30",
    "60",
    "120",
    "240",
    "360",
    "480",
    "720",
    "960",
    "D"
  ],
  "supports_group_request": false,
  "supports_marks": false,
  "supports_search": true,
  "supports_timescale_marks": false
}
```

### Symbols

**Request:**

`GET /tradingview/symbols?symbol={symbol}`

Serve information for a requested symbol

**Response:**

```json
{
  "name": "FIDA-USDC-PYTH",
  "ticker": "FIDA-USDC-PYTH",
  "description": "FIDA-USDC-PYTH",
  "type": "Spot",
  "session": "24x7",
  "exchange": "Bonfida",
  "listed_exchange": "Bonfida",
  "timezone": "Etc/UTC",
  "has_intraday": true,
  "supported_resolutions": [
    "1",
    "3",
    "5",
    "15",
    "30",
    "60",
    "120",
    "240",
    "360",
    "480",
    "720",
    "960",
    "D"
  ],
  "minmov": 1.0,
  "pricescale": 100.0
}
```

### Search

`GET /tradingview/search?query={query}&search_type={search_type}&exchange={exchange}&limit={limit}`

Can be used to search existing market

**Response:**

```json
[
  {
    "symbol": "FIDA-USDC-PYTH",
    "full_name": "FIDA-USDC-PYTH",
    "description": "Name:FIDA-USDC-PYTH - Address: ETp9eKXVv1dWwHSpsXRUuXHmw24PwRkttCGVgpZEY9zF",
    "exchange": "Bonfida",
    "ticker": "FIDA-USDC-PYTH",
    "type": "Spot"
  }
]
```

### Time

**Request:**

`GET /tradingview/time`

Returns the current unix timestamp (in seconds!) of the server

**Response:**

```json
1651188181
```

### History

**Request:**

`GET /tradingview/history?symbol={symbol}&from={from}&to={to}&resolution={resolution}`

Returns historical candles for a given symbol

**Response:**

```json
{
  "s": "ok",
  "time": [1651189320, 1651189380],
  "close": [1.2090027797967196, 1.2083083698526025],
  "open": [1.2090027797967196, 1.208549999864772],
  "high": [1.2090027797967196, 1.208549999864772],
  "low": [1.2090027797967196, 1.208055029856041],
  "volume": [0, 0]
}
```

### Pairs

`GET /tradingview/pairs`

This endpoint returns all pairs available on the server (not required by Tradingview)

<br />
<a name="deployment"></a>
<h2 align="center">Deployment</h2>
<br />

This repository can be deployed using docker compose (recommended) or natively.
