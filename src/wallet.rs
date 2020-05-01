extern crate rand;
extern crate secp256k1;

use self::secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use rand::OsRng;
use secp256k1::{Message, Secp256k1};

struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
}

impl Wallet {
    fn new_wallet() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().expect("OsRng");
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        Wallet {
            private_key: secret_key,
            public_key: public_key,
        }
    }
}

/*
let secp = Secp256k1::new();
let mut rng = OsRng::new().expect("OsRng");
let (secret_key, public_key) = secp.generate_keypair(&mut rng);
let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");

let sig = secp.sign(&message, &secret_key);
assert!(secp.verify(&message, &sig, &public_key).is_ok());
*/

/*
let secp = Secp256k1::new();
let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
let public_key = PublicKey::from_secret_key(&secp, &secret_key);
let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");

let sig = secp.sign(&message, &secret_key);
assert!(secp.verify(&message, &sig, &public_key).is_ok());
*/
