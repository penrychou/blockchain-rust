use crate::block::Block;
use crate::errors::Result;

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
        let db = sled::open("data/blocks")?;
        match db.get("LAST")? {
            Some(hash) =>{
                let last_hash = String::from_utf8(hash.to_vec())?;
                Ok(Blockchain{
                    current_hash: last_hash,
                    db,
                })

            },
            None =>{
                let block = Block::new_genesis_block();
                db.insert(block.get_hash(),bincode::serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;
                let bc = Blockchain{
                    current_hash: block.get_hash(),
                    db,
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }

    }

    pub fn add_block(&mut self, data: String) ->Result<()>{
        let last_hash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(data,String::from_utf8(last_hash.to_vec())?,4)?;
        self.db.insert(new_block.get_hash(),bincode::serialize(&new_block)?)?;
        self.db.insert("LAST",new_block.get_hash().as_bytes())?;
        Ok(())
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