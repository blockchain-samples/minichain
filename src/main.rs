extern crate crypto_hash;
use std::time::SystemTime;
use crypto_hash::{hex_digest, Algorithm};
use serde::{Serialize, Deserialize};

fn now() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Block {
    nonce: u32,
    previous_hash: String,
    timestamp: u64,
    transactions: Vec<String>,
}

impl Block {
    fn new_block(nonce: u32, previous_hash: String) -> Self {
        Block {
            nonce,
            previous_hash: previous_hash,
            timestamp: now(),
            transactions: vec![],
        }
    }
    fn hash(&self) -> String {
        hex_digest(Algorithm::SHA256, serde_json::to_string(self).unwrap().as_bytes())
    }
}

#[derive(Debug)]
struct Blockchain {
    transaction_pool: Vec<String>,
    chain: Vec<Block>
}

impl Blockchain {
    fn new_blockchain() -> Self {
        let b = Block{
            nonce: 0,
            previous_hash: "genesis block".to_string(),
            timestamp: now(),
            transactions: vec![],            
        };
        let mut bc = Blockchain {
            transaction_pool: vec![],
            chain: vec![],
        };
        bc.create_block(0, b.hash());
        return bc
    }
    fn create_block(&mut self, nonce: u32, previous_hash: String) {
        let b = Block::new_block(nonce, previous_hash);
        self.chain.push(b.clone())
    }
    fn last_block(&self) -> Block {
        self.chain[self.chain.len()-1].clone()
    }
}

fn main() {
    let mut bc = Blockchain::new_blockchain();
    let previous_hash = bc.last_block().hash();
    bc.create_block(5, previous_hash);
    let previous_hash = bc.last_block().hash();
    bc.create_block(2, previous_hash);
    println!("{:#?}", bc);
}
