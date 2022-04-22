use regex::Regex;

pub fn validate_market_name(name: &str) -> bool {
    let re = Regex::new(r"^[\w\_\-]+$").unwrap();
    re.is_match(name)
}
