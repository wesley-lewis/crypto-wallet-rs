use std::time::{SystemTime, UNIX_EPOCH};
use web3::types::U256;

pub fn get_nstime() -> u64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

pub fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;

    res / 1_000_000_000_000_000_000.0
}
