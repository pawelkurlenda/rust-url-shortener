use std::io;

use serde::{Deserialize, Serialize};

use crate::store::bptree::pager::{PageId, Pager};

#[derive(Serialize, Deserialize, Clone)]
struct Leaf {
    keys: Vec<Vec<u8>>,
    vals: Vec<Vec<u8>>,
    next: Option<PageId>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Internal {
    keys: Vec<Vec<u8>>,
    children: Vec<PageId>,
} // children len == keys+1

#[derive(Serialize, Deserialize, Clone)]
struct Meta {
    root: PageId,
    order: usize,
}

pub struct BPlusTree {
    pager: Pager,
    meta_id: PageId,
    pub root: PageId,
    pub order: usize,
}

impl BPlusTree {
    //pub fn open(mut pager: Pager, order: usize, meta_id: PageId, root: PageId) -> Self {
    pub fn open(mut pager: Pager, order: usize) -> io::Result<Self> {
        unimplemented!()
    }

    //pub fn put(&mut self, key: Vec<u8>, val: Vec<u8>) -> io::Result<()> {
    pub fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        // todo: consider to change to "insert", because updates are not allowed (rejected at design level)
        unimplemented!()
    }

    pub fn get(&mut self, key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        unimplemented!()
    }
}
