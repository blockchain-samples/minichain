use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender_blockchain_address: String,
    pub recipient_blockchain_address: String,
    pub value: f32,
}

impl Transaction {
    pub fn new_transaction(sender: String, recipient: String, value: f32) -> Self {
        Transaction {
            sender_blockchain_address: sender,
            recipient_blockchain_address: recipient,
            value: value,
        }
    }
}
