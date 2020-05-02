extern crate secp256k1;
extern crate rand;

use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl Wallet {
    pub fn new_wallet() -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Wallet {
            private_key: secret_key,
            public_key: public_key,
        }
    }
}
