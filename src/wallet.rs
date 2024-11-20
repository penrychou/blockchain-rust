use super::*;
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use bitcoincash_addr::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Wallet {
    /// NewWallet creates and returns a Wallet
    fn new() -> Self {

        let secret_key = vec![0];
        let secret_key = vec![0];

        Wallet{
            public_key,
            secret_key,
        }
        
    }

    /// GetAddress returns wallet address
    // pub fn get_address(&self) -> String {
    //     let mut pub_hash: Vec<u8> = self.public_key.clone();
    //     hash_pub_key(&mut pub_hash);
    //     let address = Address {
    //         body: pub_hash,
    //         scheme: Scheme::Base58,
    //         hash_type: HashType::Script,
    //         ..Default::default()
    //     };
    //     address.encode().unwrap()
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_wallet_and_hash() {
        let w1 = Wallet::new();
        let w2 = Wallet::new();
        assert_ne!(w1, w2);

        // let mut p2 = w2.public_key.clone();
        // hash_pub_key(&mut p2);
        // assert_eq!(p2.len(), 20);
        // let pub_key_hash = Address::decode(&w2.get_address()).unwrap().body;
        // assert_eq!(pub_key_hash, p2);
    }
}
