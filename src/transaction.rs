use serde::{Deserialize, Serialize};
use crate::errors::Result;
use sha2::{Sha256, Digest};
use crate::blockchain::Blockchain;
use failure::format_err;
use log::{debug, error, info};

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

// TXOutputs collects TXOutput
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutputs {
    pub outputs: Vec<TXOutput>,
}

/// Transaction represents a Bitcoin transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {

    pub fn new_UTXO(from: &str,to: &str,amount: i32,bc: &Blockchain) -> Result<Transaction> {
        let mut vin = Vec::new();
        let acc_v = bc.find_spendable_outputs(from,amount);
        if acc_v.0 < amount {
            error!("Not Enough balance");
            return Err(format_err!(
                "Not Enough balance: current balance {}",
                acc_v.0
            ));
        }

        for tx in acc_v.1 {
            for out in tx.1 {
                let input = TXInput {
                    txid: tx.0.clone(),
                    vout: out,
                    script_sig: String::from(from)  ,
                };
                vin.push(input);
            }
        }

        let mut vout = vec![TXOutput{
            value: amount,
            script_pub_key: String::from(to) ,
        }];

        if acc_v.0 > amount {
            vout.push(TXOutput{
                value: acc_v.0-amount,
                script_pub_key: String::from(from) ,
            })
        }

        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout,
        };
        tx.set_id()?;
        Ok(tx)
    }

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

    pub fn is_coinbase(&self) -> bool {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_signature() {
        // let mut ws = Wallets::new().unwrap();
        // let wa1 = ws.create_wallet();
        // let w = ws.get_wallet(&wa1).unwrap().clone();
        // ws.save_all().unwrap();
        // drop(ws);

        // let data = String::from("test");
        // let tx = Transaction::new_coinbase(wa1, data).unwrap();
        // assert!(tx.is_coinbase());

        // let signature = ed25519::signature(tx.id.as_bytes(), &w.secret_key);
        // assert!(ed25519::verify(tx.id.as_bytes(), &w.public_key, &signature));
    }
}