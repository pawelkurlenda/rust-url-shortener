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

    pub fn set_magic(&mut self) {
        self.buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    }

    pub fn ok_magic(&self) -> bool {
        u32::from_le_bytes(self.buf[0..4].try_into().unwrap()) == MAGIC
    }

    pub fn payload_len(&self) -> usize {
        u32::from_le_bytes(self.buf[12..16].try_into().unwrap()) as usize
    }

    pub fn set_payload_len(&mut self, n: usize) {
        self.buf[12..16].copy_from_slice(&(n as u32).to_le_bytes());
    }

    pub fn lsn(&self) -> u64 {
        u64::from_le_bytes(self.buf[16..24].try_into().unwrap())
    }

    pub fn set_lsn(&mut self, n: u64) {
        self.buf[16..24].copy_from_slice(&n.to_le_bytes());
    }

    pub fn payload_mut(&mut self) -> &mut [u8] {
        &mut self.buf[24..]
    }

    pub fn payload(&self) -> &[u8] {
        &self.buf[24..]
    }
}
