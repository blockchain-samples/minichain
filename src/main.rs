// minichain

use minichain::blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new_blockchain();
    bc.add_transaction("A".to_string(), "B".to_string(), 1.0);
    let mut previous_hash = bc.last_block().hash();
    bc.create_block(5, previous_hash);
    bc.add_transaction("C".to_string(), "D".to_string(), 2.0);
    bc.add_transaction("X".to_string(), "Y".to_string(), 3.0);
    previous_hash = bc.last_block().hash();
    bc.create_block(2, previous_hash);
    println!("{:#?}", bc);
}
