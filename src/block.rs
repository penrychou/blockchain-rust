use super::*;
use crate::transaction::Transaction;
use bincode::serialize;
use sha2::{Sha256, Digest};
use merkle_cbt::merkle_tree::Merge;
use merkle_cbt::merkle_tree::CBMT;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use log::info;
use sha2::digest::Update;

const TARGET_HEXS: usize = 4;

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Block {
    timestamp: u128,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    nonce: i32,
    height: usize,
}

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    pub fn get_transactions(&self) -> String {
        self.transactions.clone()
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /// NewBlock creates and returns Block
    pub fn new_block(
        data: String,
        prev_block_hash: String,
        height: usize,
    ) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let mut block = Block {
            timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
            height,
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    /// NewGenesisBlock creates and returns genesis Block
    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("Gensis block"), String::new(), 0).unwrap()
    }

    /// Run performs a proof-of-work
    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
        }
        let mut data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        sha2::Digest::update(&mut hasher,&data);
        let hex_result = hasher.finalize().iter().map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("");
        self.hash = hex_result.clone();
        Ok(())
    }

    /// HashTransactions returns a hash of the transactions in the block
    // fn hash_transactions(&self) -> Result<Vec<u8>> {
    //     let mut transactions = Vec::new();
    //     for tx in &self.transactions {
    //         transactions.push(tx.hash()?.as_bytes().to_owned());
    //     }
    //     let tree = CBMT::<Vec<u8>, MergeVu8>::build_merkle_tree(&transactions);
    //
    //     Ok(tree.root())
    // }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXS,
            self.nonce,
        );
        let bytes = serialize(&content)?;
        Ok(bytes)
    }

    /// Validate validates block's PoW
    fn validate(&self) -> Result<bool> {
        let mut data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        sha2::Digest::update(&mut hasher,&data);
        let hex_result = hasher.finalize().iter().map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("");
        let mut vec1: Vec<u8> = Vec::new();
        vec1.resize(TARGET_HEXS, '0' as u8);
        Ok(hex_result[0..TARGET_HEXS] == String::from_utf8(vec1)?)
    }
    
}