use serde::{Deserialize, Serialize};
use crate::errors::Result;
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub script_sig: String,
}

/// TXOutput represents a transaction output
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub script_pub_key: String,
}

/// Transaction represents a Bitcoin transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {
    pub fn new_coinbase(to: String,mut data: String) ->Result<Transaction>{
        if data == String::from(""){
            data += &format!("Reward to '{}'",to);
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![TXInput {
                txid: String::new(),
                vout: -1,
                script_sig: data,
            }],
            vout: vec![TXOutput{
                value: 100,
                script_pub_key: to,
            }],
        };
        tx.set_id()?;
        Ok(tx)
    }

    pub fn hash(&self) -> Result<String> {
        let mut hasher = Sha256::new();
        let data = bincode::serialize(self)?;
        sha2::Digest::update(&mut hasher,&data);
        let hex_result = hasher.finalize().iter().map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("");
        Ok(hex_result)
    }

    fn set_id(&mut self) -> Result<()> {
        let mut hasher = Sha256::new();
        let data = bincode::serialize(self)?;
        sha2::Digest::update(&mut hasher,&data);
        let hex_result = hasher.finalize().iter().map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("");

        self.id = hex_result;
        Ok(())

    }

    pub fn is_coinbase(&mut self) -> bool {
        self.vin.len()==1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }
}

impl TXInput{
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig == unlocking_data
    }
}

impl TXOutput{
    pub fn can_be_unlock_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}