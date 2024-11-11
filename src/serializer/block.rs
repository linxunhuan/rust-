/*通过哈希计算，计算区块中的hash，prehash和txhash
此处我用字符串来模拟交易，并通过将其放入Vec中来实现多笔交易
*/

use std::thread;
use std::time::Duration;
use chrono::prelude::*;
use crate::serializer::{serialize,hash_str};
use serde::Serialize;

// 区块头结构体
#[derive(Serialize,Debug,PartialEq,Eq)]
pub struct BlockHeader {
    pub time:i64,
    pub pre_hash:String,
    pub txs_hash:String,
}

// 区块结构体
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub tranxs:String,
    pub hash:String,
}

impl Block {
    pub fn new(txs:String,pre_hash:String)->Self{
        // 延迟三秒再挖
        println!("Start mining...");
        thread::sleep(Duration::from_secs(3));
        
        // 准备时间，计算交易哈希值
        let time = Utc::now().timestamp();
        let txs_hash = serialize(&txs);
        let txs_hash = hash_str(&txs_hash);
        let mut block = Block{
            header: BlockHeader{
                time:time,
                txs_hash:txs_hash,
                pre_hash:pre_hash,
            },
            tranxs:txs,
            hash:"".to_string(),
        }; 
        block.set_hash();
        println!("produce a new block!\n");
        return block;
    }
    
    // 计算并设置区块哈希值
    fn set_hash(&mut self){
        let header_str = serialize(&self.header);
        self.hash = hash_str(&header_str);
    }
}