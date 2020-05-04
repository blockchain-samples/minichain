extern crate secp256k1;
//extern crate crypto_hash;
extern crate bitcoin_hashes;

use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey};
//use crypto_hash::{hex_digest, Algorithm};
use bitcoin_hashes::{sha256, Hash, ripemd160};

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: String,
}

impl Wallet {
    pub fn new_wallet() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().unwrap();
        // 1. Having a private ECDSA key.
        // 2. Take the corresponding public key generated with it (33 bytes, 1 byte 0x02 (y-coord is even), and 32 bytes corresponding to X coordinate)
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        // 3. Perform SHA-256 hashing on the public key
        let h3 = sha256::Hash::hash(&public_key.serialize());
        // 4. Perform RIPEMD-160 hashing on the result of SHA-256
        let h4 = ripemd160::Hash::hash(&h3).into_inner();
        // 5. Add version byte in front of RIPEMD-160 hash (0x00 for Main Network)
        let mut h5 = [0u8; 1].to_vec();
        h5.extend(&h4);
        // 6. Perform SHA-256 hash on the extended RIPEMD-160 result
        let h6 = sha256::Hash::hash(&h5);
        // 7. Perform SHA-256 hash on the result of the previous SHA-256 hash
        let h7 = sha256::Hash::hash(&h6);
        // 8. Take the first 4 bytes of the second SHA-256 hash. This is the address checksum
        let h8 = &h7[..4];
        // 9. Add the 4 checksum bytes from stage 7 at the end of extended RIPEMD-160 hash from stage 4. This is the 25-byte binary Bitcoin Address.
        let mut h9 = h5.clone();
        h9.extend(h8);
        // 10. Convert the result from a byte string into a base58 string using Base58Check encoding. This is the most commonly used Bitcoin Address format
        let h10 = bs58::encode(&h9).into_string();

        Wallet {
            private_key: secret_key,
            public_key: h10,
        }
    }
}
