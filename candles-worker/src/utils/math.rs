use std::ops::Div;

pub fn to_f64(fp32_n: u64) -> f64 {
    (fp32_n as f64).div((1u64 << 32) as f64)
}
