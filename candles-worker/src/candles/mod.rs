use crate::utils::time::current_time;
pub struct Candle {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub ts_start: i64,
}

impl Candle {
    pub fn new(initial_value: f64) -> Self {
        let now = current_time() as i64;
        Self {
            open: initial_value,
            close: initial_value,
            high: initial_value,
            low: initial_value,
            ts_start: now - (now % 60), // Round to closest second
        }
    }
}
