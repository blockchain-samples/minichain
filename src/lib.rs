mod block;
pub mod blockchain;
mod transaction;
pub mod wallet;

use std::time::SystemTime;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
