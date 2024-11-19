use crate::block::Block;
use crate::errors::Result;
use log::{debug, info};
use bincode::{deserialize, serialize};
use crate::transaction::Transaction;

#[derive(Debug,Clone)]
pub struct Blockchain{
    // blocks: Vec<Block>
    current_hash: String,
    db: sled::Db,
}

pub struct BlockchainIter<'a>{
    // blocks: Vec<Block>
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {

    pub fn new() -> Result<Blockchain> {
        info!("open blockchain");

        let db = sled::open("data/blocks")?;
        let hash = match db.get("LAST")? {
            Some(l) => l.to_vec(),
            None => Vec::new(),
        };
        info!("Found block database");
        let lasthash = if hash.is_empty() {
            String::new()
        } else {
            String::from_utf8(hash.to_vec())?
        };
        Ok(Blockchain { current_hash: lasthash, db })
    }


    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating new blockchain");

        std::fs::remove_dir_all("data/blocks").ok();
        let db = sled::open("data/blocks")?;
        debug!("Creating new block database");
        let cbtx = Transaction::new_coinbase(address, String::from("GENESIS_COINBASE_DATA"))?;
        let genesis: Block = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            current_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
    }


    pub fn add_block(&mut self, block: Block) ->Result<()>{
        let data = serialize(&block)?;
        if let Some(_) = self.db.get(block.get_hash())? {
            return Ok(());
        }

        self.db.insert(block.get_hash(), data)?;
        let lastheight = self.get_best_height()?;
        if block.get_height() > lastheight {
            self.db.insert("LAST", block.get_hash().as_bytes())?;
            self.current_hash = block.get_hash();
            self.db.flush()?;
        }
        Ok(())
    }

    // GetBlock finds a block by its hash and returns it
    pub fn get_block(&self, block_hash: &str) -> Result<Block> {
        let data = self.db.get(block_hash)?.unwrap();
        let block = deserialize(&data.to_vec())?;
        Ok(block)
    }

    /// GetBestHeight returns the height of the latest block
    pub fn get_best_height(&self) -> Result<i32> {
        let lasthash = if let Some(h) = self.db.get("LAST")? {
            h
        } else {
            return Ok(-1);
        };
        let last_data = self.db.get(lasthash)?.unwrap();
        let last_block: Block = deserialize(&last_data.to_vec())?;
        Ok(last_block.get_height())
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter{current_hash: self.current_hash.clone(), bc: &self, }
    }

}


impl<'a>  Iterator for BlockchainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item>{
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash){
            return match encode_block {
                Some(b) =>{
                    if let Ok(block) = bincode::deserialize::<Block>(&b){
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    }else{
                        None
                    }
                }
                None => None
            };
        }
        None
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blockchain(){
        let mut b = Blockchain::new().unwrap();
        b.add_block("data1".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string());


        for item in b.iter(){
            println!("item: {:?}", item);
        }
    }

}