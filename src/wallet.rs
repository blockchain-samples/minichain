extern crate bitcoin_hashes;
extern crate hex;
extern crate secp256k1;

use bitcoin_hashes::{ripemd160, sha256, Hash};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    pub blockchain_address: String,
}

impl Wallet {
    pub fn new_wallet() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().unwrap();

        // 1. Having a private ECDSA key.
        // 2. Take the corresponding public key generated with it.
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);

        // 3. Perform SHA-256 hashing on the public key.
        let sha256_public_key = sha256::Hash::hash(&public_key.serialize());

        // 4. Perform RIPEMD-160 hashing on the result of SHA-256.
        let ripmd160_sha256_public_key = ripemd160::Hash::hash(&sha256_public_key).into_inner();

        // 5. Add version byte in front of RIPEMD-160 hash (0x00 for Main Network).
        let mut extended_ripmd160_sha256_public_key = [0u8; 1].to_vec();
        extended_ripmd160_sha256_public_key.extend(&ripmd160_sha256_public_key);

        // 6. Perform SHA-256 hash on the extended RIPEMD-160 result.
        let sha256_extended_ripmd160_sha256_public_key =
            sha256::Hash::hash(&extended_ripmd160_sha256_public_key);

        // 7. Perform SHA-256 hash on the result of the previous SHA-256 hash.
        let double_sha256_extended_ripmd160_sha256_public_key =
            sha256::Hash::hash(&sha256_extended_ripmd160_sha256_public_key);

        // 8. Take the first 4 bytes of the second SHA-256 hash.
        // This is the address checksum.
        let checksum = &double_sha256_extended_ripmd160_sha256_public_key[..4];

        // 9. Add the 4 checksum bytes from stage 7 at the end of extended RIPEMD-160 hash from stage 4.
        // This is the 25-byte binary Bitcoin Address.
        let mut binary_bitcoin_address = extended_ripmd160_sha256_public_key.clone();
        binary_bitcoin_address.extend(checksum);

        // 10. Convert the result from a byte string into a base58 string using Base58Check encoding.
        // This is the most commonly used Bitcoin Address format.
        let bitcoin_address = bs58::encode(&binary_bitcoin_address).into_string();

        Wallet {
            private_key: secret_key,
            public_key: public_key,
            blockchain_address: bitcoin_address,
        }
    }
}

pub struct Transaction {
    sender_private_key: SecretKey,
    sender_public_key: PublicKey,
    sender_blockchain_address: String,
    recipient_blockchain_address: String,
    value: f32,
}

impl Transaction {
    pub fn new_transaction(
        sender_private_key: SecretKey,
        sender_public_key: PublicKey,
        sender_blockchain_address: String,
        recipient_blockchain_address: String,
        value: f32,
    ) -> Self {
        Transaction {
            sender_private_key: sender_private_key,
            sender_public_key: sender_public_key,
            sender_blockchain_address: sender_blockchain_address,
            recipient_blockchain_address: recipient_blockchain_address,
            value: value,
        }
    }
    pub fn generate_signature(&self) -> Signature {
        let secp = Secp256k1::new();
        let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
        secp.sign(&message, &self.sender_private_key)
    }
}
