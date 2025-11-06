use crate::store::bptree::pager::{PAGE_SIZE, PageId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageKind {
    Meta = 0,
    Internal = 1,
    Leaf = 2,
}

pub const MAGIC: u32 = 0x4250_5445;

#[derive(Clone)]
pub struct Page {
    pub id: PageId,
    pub buf: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new(page_id: PageId) -> Self {
        Self {
            id: page_id,
            buf: [0u8; PAGE_SIZE],
        }
    }
}
