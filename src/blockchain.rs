use crate::block::Block;
use crate::now;

#[derive(Debug)]
pub struct Blockchain {
    transaction_pool: Vec<String>,
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new_blockchain() -> Self {
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
    pub fn create_block(&mut self, nonce: u32, previous_hash: String) {
        let b = Block::new_block(nonce, previous_hash);
        self.chain.push(b)
    }
    pub fn last_block(&self) -> &Block {
        &self.chain[self.chain.len()-1]
    }
}
