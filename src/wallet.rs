extern crate secp256k1;
//extern crate crypto_hash;
extern crate bitcoin_hashes;

use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
//use crypto_hash::{hex_digest, Algorithm};
use bitcoin_hashes::{sha256, Hash, ripemd160};

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: [u8;20],
}

impl Wallet {
    pub fn new_wallet() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().unwrap();
        // 0. Having a private ECDSA key.
        // 1. Take the corresponding public key generated with it.
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        // 2. Perform SHA-256 hashing on the public key.
        // let digest = hex_digest(Algorithm::SHA256, &public_key.serialize());
        let h2 = sha256::Hash::hash(&public_key.serialize());
        // 3. Perform RIPEMD-160 hashing on the result of SHA-256.
        let h3 = ripemd160::Hash::hash(&h2).into_inner();
        // 4. Add version byte in front of RIPEMD-160 hash.
        let mut h4 = [0u8; 2].to_vec();
        h4.extend(&h3);
        // 5. Perform SHA-256 hash on the extended RIPEMD-160 result.
        let h5 = sha256::Hash::hash(&h4);
        // 6. Perform SHA-256 hash on the result of the previous SHA-256 hash.
        let h6 = sha256::Hash::hash(&h5);
        // 7. Take the first 4 bytes of the second SHA-256 hash.
        let h7 = &h6[..4];
        // 8. Add the 4 checksum bytes from stage 7 at the end of extended RIPEMD-160 hash from stage 4.
        h7.to_vec().extend(&h4);
        // 9. Convert the result from a byte string into a base58 string using Base58Check encoding.
        Wallet {
            private_key: secret_key,
            public_key: h3,
        }
    }
}
