use crate::{
    error::ServerError,
    structs::{candles::Candle, raw_market::RawMarket},
    utils::time::from_timestampz,
};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct Config {
    pub supported_resolutions: [Resolution; 13],
    pub supports_group_request: bool,
    pub supports_marks: bool,
    pub supports_search: bool,
    pub supports_timescale_marks: bool,
}

#[derive(Serialize)]
pub struct Symbol {
    pub symbol: String,
    pub full_name: String,
    pub description: String,
    pub exchange: String,
    pub ticker: String,
    /// Always Spot in practice
    #[serde(rename(serialize = "type"))]
    pub symbol_type: String,
}

#[derive(Serialize)]
pub struct Bar {
    /// ok, error, no_data
    #[serde(rename(serialize = "s"))]
    pub status: String,
    #[serde(rename(serialize = "errmsg"), skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub time: Vec<u64>,
    pub close: Vec<f64>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub volume: Vec<u64>,
    /// Only Some if s == no_data
    #[serde(
        rename(serialize = "nextTime"),
        skip_serializing_if = "Option::is_none"
    )]
    pub next_time: Option<u64>,
}

impl Bar {
    pub fn candles_to_bar(candles: Vec<Candle>) -> Self {
        let mut time: Vec<u64> = Vec::new();
        let mut close: Vec<f64> = Vec::new();
        let mut open: Vec<f64> = Vec::new();
        let mut low: Vec<f64> = Vec::new();
        let mut high: Vec<f64> = Vec::new();

        for c in candles.into_iter() {
            time.push(from_timestampz(c.ts_start) as u64);
            close.push(c.close);
            open.push(c.open);
            high.push(c.high);
            low.push(c.low);
        }

        // Debug checks
        assert_eq!(time.len(), close.len());
        assert_eq!(close.len(), open.len());
        assert_eq!(open.len(), low.len());
        assert_eq!(low.len(), high.len());

        let len = time.len();

        Bar {
            status: "ok".to_owned(),
            error_message: None,
            time,
            close,
            open,
            low,
            high,
            volume: vec![0u64; len],
            next_time: None,
        }
    }
}

#[derive(Serialize)]
pub struct SymbolInfo {
    pub name: String,
    pub ticker: String,
    pub description: String,
    #[serde(rename(serialize = "type"))]
    pub symbol_type: String,
    /// "24x7"
    pub session: String,
    pub exchange: String,
    pub listed_exchange: String,
    /// "Etc/UTC"
    pub timezone: String,
    /// true
    pub has_intraday: bool,
    pub supported_resolutions: [Resolution; 13],
    #[serde(rename(serialize = "minmov"))]
    pub min_mov: f64,
    #[serde(rename(serialize = "pricescale"))]
    pub price_scale: f64,
}

impl SymbolInfo {
    pub fn new(symbol: &str, raw_markets: &[RawMarket]) -> Result<Self, ServerError> {
        let market = raw_markets
            .iter()
            .find(|x| x.name == symbol)
            .ok_or(ServerError::RawMarketNotFound)?;

        Ok(Self {
            name: symbol.to_string(),
            ticker: symbol.to_string(),
            description: symbol.to_string(),
            symbol_type: "Spot".to_owned(),
            session: "24x7".to_owned(),
            exchange: "Bonfida".to_owned(),
            listed_exchange: "Bonfida".to_owned(),
            timezone: "Etc/UTC".to_owned(),
            has_intraday: true,
            supported_resolutions: RESOLUTIONS,
            min_mov: market.min_mov,
            price_scale: market.price_scale,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Resolution {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    SixteenHours,
    OneDay,
}

impl Resolution {
    pub fn as_str<'a>(self) -> &'a str {
        match self {
            Resolution::OneMinute => "1",
            Resolution::ThreeMinutes => "3",
            Resolution::FiveMinutes => "5",
            Resolution::FifteenMinutes => "15",
            Resolution::ThirtyMinutes => "30",
            Resolution::OneHour => "60",
            Resolution::TwoHours => "120",
            Resolution::FourHours => "240",
            Resolution::SixHours => "360",
            Resolution::EightHours => "480",
            Resolution::TwelveHours => "720",
            Resolution::SixteenHours => "960",
            Resolution::OneDay => "D",
        }
    }

    pub fn from_str(v: &str) -> Result<Self, ()> {
        match v {
            "1" => Ok(Resolution::OneMinute),
            "3" => Ok(Resolution::ThreeMinutes),
            "5" => Ok(Resolution::FiveMinutes),
            "15" => Ok(Resolution::FifteenMinutes),
            "30" => Ok(Resolution::ThirtyMinutes),
            "60" => Ok(Resolution::OneHour),
            "120" => Ok(Resolution::TwoHours),
            "240" => Ok(Resolution::FourHours),
            "360" => Ok(Resolution::SixHours),
            "480" => Ok(Resolution::EightHours),
            "720" => Ok(Resolution::TwelveHours),
            "960" => Ok(Resolution::SixteenHours),
            "D" => Ok(Resolution::OneDay),
            _ => Err(()),
        }
    }

    pub fn to_interval(self) -> String {
        match self {
            Resolution::OneDay => "1440 minutes".to_owned(),
            other => format!("{} minutes", other.as_str()),
        }
    }
}

pub const RESOLUTIONS: [Resolution; 13] = [
    Resolution::OneMinute,
    Resolution::ThreeMinutes,
    Resolution::FiveMinutes,
    Resolution::FifteenMinutes,
    Resolution::ThirtyMinutes,
    Resolution::OneHour,
    Resolution::TwoHours,
    Resolution::FourHours,
    Resolution::SixHours,
    Resolution::EightHours,
    Resolution::TwelveHours,
    Resolution::SixteenHours,
    Resolution::OneDay,
];

impl Serialize for Resolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
