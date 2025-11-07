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

    pub fn read_kind(&self) -> PageKind {
        match self.buf[8] {
            1 => PageKind::Internal,
            2 => PageKind::Leaf,
            _ => PageKind::Meta,
        }
    }

    pub fn write_kind(&mut self, k: PageKind) {
        self.buf[8] = match k {
            PageKind::Meta => 0,
            PageKind::Internal => 1,
            PageKind::Leaf => 2,
        };
    }
}
