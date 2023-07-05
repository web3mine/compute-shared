use crate::types::chain::block::Block;
use crate::CID;
use serde::{Deserialize, Serialize};

use super::cid::cidr2str;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChainHead {
    pub Height: i64,
    pub Blocks: Vec<Block>,
    pub Cids: Vec<CID>,
}

impl ChainHead {
    pub fn key(&self) -> Option<String> {
        let mut buffer: Vec<u8> = Vec::new();
        for cid in self.Cids.iter() {
            let mut cid_as_bytes = cidr2str(cid)?.as_bytes().to_vec();
            buffer.append(&mut cid_as_bytes)
        }
        String::from_utf8(buffer).ok()
    }
}
