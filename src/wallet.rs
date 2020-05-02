extern crate secp256k1;

use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl Wallet {
    pub fn new_wallet() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().unwrap();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);

        Wallet {
            private_key: secret_key,
            public_key: public_key,
        }
    }
}
