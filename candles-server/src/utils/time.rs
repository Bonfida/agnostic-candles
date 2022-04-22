use {
    chrono::{NaiveDateTime, Utc},
    std::time::SystemTime,
};

/// Returns the current unix timestamp in seconds
pub fn current_time() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn to_timestampz(seconds: u64) -> chrono::DateTime<Utc> {
    chrono::DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(seconds as i64, 0), Utc)
}

pub fn from_timestampz(ts: chrono::DateTime<Utc>) -> i64 {
    chrono::DateTime::<Utc>::timestamp(&ts)
}
