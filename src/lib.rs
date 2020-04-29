mod block;
pub mod blockchain;
mod transaction;

use std::time::SystemTime;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
