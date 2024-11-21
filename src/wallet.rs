use super::*;
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use bitcoincash_addr::*;
use sha2::{Sha256, Digest};
use bitcoin_hashes::{ripemd160, Hash};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand_core::OsRng; 


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wallet {
    secret_key: Vec<u8>,
    public_key: Vec<u8>,
    pub address: String
}

impl Wallet {
    /// NewWallet creates and returns a Wallet
    fn new() -> Self {

        // 生成随机的ED25519密钥对
        let signing_key = SigningKey::generate(&mut OsRng); ;
        
        // 获取公钥和私钥
        let public_key = signing_key.verifying_key().as_bytes().to_vec();
        let secret_key = signing_key.as_bytes().to_vec();
        
        // 生成比特币地址
        let address = Self::generate_address(&public_key);
        
        Wallet {
            secret_key,
            public_key,
            address,
        }
        
    }

    fn generate_address(public_key: &Vec<u8>) -> String {
        // 1. SHA256(公钥)
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(public_key);
        let sha256_result = sha256_hasher.finalize();
        
        // 2. RIPEMD160(SHA256(公钥))
        let mut ripemd_result  = ripemd160::Hash::hash(&sha256_result);

        
         // 3. 添加版本号前缀 (0x00 为主网地址)
        let mut version_payload = vec![0x00];
        version_payload.extend_from_slice(&ripemd_result[..]);
        
        // 4. 双重SHA256用于校验
        let mut check_hasher = Sha256::new();
        check_hasher.update(&version_payload);
        let mut check_result = check_hasher.finalize();
        
        let mut check_hasher2 = Sha256::new();
        check_hasher2.update(&check_result);
        check_result = check_hasher2.finalize();
        
        // 5. 取前4字节作为校验和
        let checksum = &check_result[0..4];
        
        // 6. 将校验和附加到payload
        version_payload.extend_from_slice(checksum);
        
        // 7. Base58编码
        bs58::encode(version_payload).into_string()
    }

    /// GetAddress returns wallet address
    pub fn get_address(&self) -> &String {
        &self.address
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_wallet_and_hash() {
        let w1 = Wallet::new();
        println!("Bitcoin Address1: {}", w1.get_address());
        let w2 = Wallet::new();
        println!("Bitcoin Address2: {}", w2.get_address());
        assert_ne!(w1, w2);
    }
}
