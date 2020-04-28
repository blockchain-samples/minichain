use minichain::blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new_blockchain();
    let previous_hash = bc.last_block().hash();
    bc.create_block(5, previous_hash);
    let previous_hash = bc.last_block().hash();
    bc.create_block(2, previous_hash);
    println!("{:#?}", bc);
}
