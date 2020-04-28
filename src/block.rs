extern crate crypto_hash;
use crypto_hash::{hex_digest, Algorithm};
use serde::{Serialize, Deserialize};
use crate::now;
use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub nonce: u32,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new_block(nonce: u32, previous_hash: String, transactions: &Vec<Transaction>) -> Self {
        Block {
            nonce,
            previous_hash: previous_hash,
            timestamp: now(),
            transactions: transactions.to_vec(),
        }
    }
    pub fn hash(&self) -> String {
        hex_digest(Algorithm::SHA256, serde_json::to_string(self).unwrap().as_bytes())
    }
}
