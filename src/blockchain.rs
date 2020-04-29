use crate::block::Block;
use crate::now;
use crate::transaction::Transaction;

const MINING_DIFFICULTY: usize = 3;
const MINING_SENDER: &str = "THE BLOCKCHAIN";
const MINING_REWARD: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct Blockchain {
    transaction_pool: Vec<Transaction>,
    chain: Vec<Block>,
    blockchain_address: String,
}

impl Blockchain {
    pub fn new_blockchain(blockchain_address: String) -> Self {
        let b = Block {
            nonce: 0,
            previous_hash: "genesis block".to_string(),
            timestamp: now(),
            transactions: vec![],
        };
        let mut bc = Blockchain {
            transaction_pool: vec![],
            chain: vec![],
            blockchain_address: blockchain_address,
        };
        bc.create_block(0, b.hash());
        return bc;
    }
    pub fn create_block(&mut self, nonce: u32, previous_hash: String) {
        let b = Block::new_block(nonce, previous_hash, &self.transaction_pool);
        self.chain.push(b);
        self.transaction_pool = vec![]
    }
    pub fn last_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }
    pub fn add_transaction(&mut self, sender: String, recipient: &String, value: f32) {
        let t = Transaction::new_transaction(sender, recipient.to_string(), value);
        self.transaction_pool.push(t)
    }
    fn valid_proof(
        &self,
        nonce: u32,
        previous_hash: &String,
        transactions: &Vec<Transaction>,
        difficulty: usize,
    ) -> bool {
        let zeros = "000".to_string();
        let guess_block = Block {
            nonce: nonce,
            previous_hash: previous_hash.to_string(),
            timestamp: 0,
            transactions: transactions.to_vec(),
        };
        let guess_hash_str = guess_block.hash();
        guess_hash_str[..difficulty] == zeros
    }
    pub fn proof_of_work(&self) -> u32 {
        let transactions = self.transaction_pool.clone();
        let previous_hash = self.last_block().hash();
        let mut nonce = 0;
        while self.valid_proof(nonce, &previous_hash, &transactions, MINING_DIFFICULTY) != true {
            nonce += 1;
        }
        nonce
    }
    pub fn mining(&mut self) -> bool {
        self.add_transaction(
            MINING_SENDER.to_string(),
            &self.blockchain_address.to_string(),
            MINING_REWARD,
        );
        let nonce = self.proof_of_work();
        let previous_hash = self.last_block().hash();
        self.create_block(nonce, previous_hash);
        true
    }
}
