//有了区块后，接下来就是构建区块链，用Vec存储多个区块

use crate::block::Block;
use crate::serializer::block::Block;

// 第一个区块没有prehash，所以需要手动设置
const PRE_HASH: &str = "UnVzdCBsZWFybmluZyBpbiBCbG9jaw==";

pub struct Blockchain {
    pub blocks: Vec<Block>,
}-

impl Blockchain{
    pub fn new() -> Self {
        Blockchain{ blocks:vec![Self::genesis_block()]}
    }
    
    // 生成创世区块
    fn genesis_block() -> Block {
        Block::("创世区块".to_string(),PRE_HASH.to_string())
    }
    
    // 添加区块，形成区块链
    pub fn add_block(&mut self,data:String){
        // 获取前一个区块的hash值
        let pre_block = &self.blocks[self.blocks.len()-1];
        let pre_hash = pre_block.hash().clone();
        
        // 构建新区块并加入区块链
        let new_block = Block::new(data,pre_hash);
        self.blocks.push(new_block);
    }
    
    // 输出区块信息
    pub fn block_info(&self){
        for b in self.blocks.iter(){
            println!("{:#?}",b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blockchain() {
        println!("----------------------Mine info---------------------------------");
        let mut bc = Blockchain::new();

        let tx = "0xabcd->0xabce:5 btc".to_string();
        bc.add_block(tx);
        let tx = "0xabce->0xabcf:10 btc".to_string();
        bc.add_block(String::from(tx));
        println!("----------------------Block info---------------------------------");
        bc.block_info();
    }
}