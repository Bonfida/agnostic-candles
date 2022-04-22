use std::time::SystemTime;

/// Returns the current unix timestamp in seconds
pub fn current_time() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
