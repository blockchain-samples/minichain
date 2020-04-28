use crate::block::Block;
use crate::now;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    transaction_pool: Vec<Transaction>,
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
        let b = Block::new_block(nonce, previous_hash, &self.transaction_pool);
        self.chain.push(b);
        self.transaction_pool = vec!()
    }
    pub fn last_block(&self) -> &Block {
        &self.chain[self.chain.len()-1]
    }
    pub fn add_transaction(&mut self, sender: String, recipient: String, value: f32) {
        let t = Transaction::new_transaction(sender, recipient, value);
        self.transaction_pool.push(t)
    }
}
