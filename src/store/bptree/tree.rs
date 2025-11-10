use std::io;

use serde::{Deserialize, Serialize};

use crate::store::bptree::{
    page::{Page, PageKind},
    pager::{PAGE_SIZE, PageId, Pager},
};

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
        let pages = pager.len_pages()?;
        if pages == 0 {
            let meta_id = pager.allocate()?; // 0
            let root_id = pager.allocate()?; // 1

            let mut p = Page::new(root_id);
            p.set_magic();
            p.write_kind(PageKind::Leaf);
            let leaf = Leaf {
                keys: vec![],
                vals: vec![],
                next: None,
            };
            let bytes = bincode::serialize(&leaf).unwrap();
            assert!(bytes.len() <= PAGE_SIZE - 24);
            p.set_payload_len(bytes.len());
            p.payload_mut()[..bytes.len()].copy_from_slice(&bytes);
            pager.write_page(root_id, &p.buf)?;

            let mut m = Page::new(meta_id);
            m.set_magic();
            m.write_kind(PageKind::Meta);
            let meta = Meta {
                root: root_id,
                order: order.max(4),
            };
            let mb = bincode::serialize(&meta).unwrap();
            m.set_payload_len(mb.len());
            m.payload_mut()[..mb.len()].copy_from_slice(&mb);
            pager.write_page(meta_id, &m.buf)?;

            Ok(Self {
                pager,
                meta_id,
                root: root_id,
                order: order.max(4),
            })
        } else {
            let meta_id = 0;
            let mut buf = [0u8; PAGE_SIZE];
            pager.read_page(meta_id, &mut buf)?;
            let payload_len = u32::from_le_bytes(buf[12..16].try_into().unwrap()) as usize;
            let meta: Meta = bincode::deserialize(&buf[24..24 + payload_len]).unwrap();
            Ok(Self {
                pager,
                meta_id,
                root: meta.root,
                order: meta.order,
            })
        }
    }

    // todo: consider to change to "insert", because updates are not allowed (rejected at design level)
    //pub fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
    pub fn put(&mut self, key: Vec<u8>, val: Vec<u8>) -> io::Result<()> {
        let res = self.insert_rec(self.root, key, val)?;
        if let Some((sep, right_id)) = res {
            let new_root = self.pager.allocate()?;
            let node = Internal {
                keys: vec![sep],
                children: vec![self.root, right_id],
            };
            self.write_internal(new_root, &node)?;
            self.root = new_root;
            self.save_meta()?;
        }
        Ok(())
    }

    pub fn get(&mut self, key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        let mut cur = self.root;
        loop {
            let mut buf = [0u8; PAGE_SIZE];
            self.pager.read_page(cur, &mut buf)?;
            match buf[8] {
                1 => {
                    // internal
                    let payload_len = u32::from_le_bytes(buf[12..16].try_into().unwrap()) as usize;
                    let node: Internal = bincode::deserialize(&buf[24..24 + payload_len]).unwrap();
                    let mut idx = match node.keys.binary_search_by(|k| k.as_slice().cmp(key)) {
                        Ok(i) => i + 1,
                        Err(i) => i,
                    };
                    if idx >= node.children.len() {
                        idx = node.children.len() - 1;
                    }
                    cur = node.children[idx];
                }
                2 => {
                    // leaf
                    let payload_len = u32::from_le_bytes(buf[12..16].try_into().unwrap()) as usize;
                    let leaf: Leaf = bincode::deserialize(&buf[24..24 + payload_len]).unwrap();
                    if let Ok(i) = leaf.keys.binary_search_by(|k| k.as_slice().cmp(key)) {
                        let v = leaf.vals[i].clone();
                        return if v.is_empty() { Ok(None) } else { Ok(Some(v)) };
                    } else {
                        return Ok(None);
                    }
                }
                _ => return Ok(None),
            }
        }
    }

    fn save_meta(&mut self) -> io::Result<()> {
        let mut buf = [0u8; PAGE_SIZE];
        self.pager.read_page(self.meta_id, &mut buf)?;
        let mut p = Page {
            id: self.meta_id,
            buf,
        };
        let meta = Meta {
            root: self.root,
            order: self.order,
        };
        let mb = bincode::serialize(&meta).unwrap();
        p.set_payload_len(mb.len());
        p.payload_mut()[..mb.len()].copy_from_slice(&mb);
        self.pager.write_page(self.meta_id, &p.buf)
    }

    fn insert_rec(
        &mut self,
        node_id: PageId,
        key: Vec<u8>,
        val: Vec<u8>,
    ) -> io::Result<Option<(Vec<u8>, PageId)>> {
        let mut buf = [0u8; PAGE_SIZE];
        self.pager.read_page(node_id, &mut buf)?;
        match buf[8] {
            2 => {
                // leaf
                let mut leaf = {
                    let payload_len = u32::from_le_bytes(buf[12..16].try_into().unwrap()) as usize;
                    bincode::deserialize::<Leaf>(&buf[24..24 + payload_len]).unwrap()
                };
                match leaf.keys.binary_search_by(|k| k.as_slice().cmp(&key)) {
                    Ok(i) => {
                        leaf.vals[i] = val;
                        self.write_leaf(node_id, &leaf)?;
                        return Ok(None);
                    }
                    Err(i) => {
                        leaf.keys.insert(i, key);
                        leaf.vals.insert(i, val);
                    }
                }
                if leaf.keys.len() > self.order {
                    let mid = leaf.keys.len() / 2;
                    let right_keys = leaf.keys.split_off(mid);
                    let right_vals = leaf.vals.split_off(mid);
                    let sep = right_keys[0].clone();
                    let right_id = self.pager.allocate()?;
                    let right = Leaf {
                        keys: right_keys,
                        vals: right_vals,
                        next: leaf.next,
                    };
                    leaf.next = Some(right_id);
                    self.write_leaf(node_id, &leaf)?;
                    self.write_leaf(right_id, &right)?;
                    Ok(Some((sep, right_id)))
                } else {
                    self.write_leaf(node_id, &leaf)?;
                    Ok(None)
                }
            }
            1 => {
                // internal
                let mut node = {
                    let payload_len = u32::from_le_bytes(buf[12..16].try_into().unwrap()) as usize;
                    bincode::deserialize::<Internal>(&buf[24..24 + payload_len]).unwrap()
                };
                let idx = match node.keys.binary_search_by(|k| k.as_slice().cmp(&key)) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                };
                let child = node.children[idx];
                if let Some((sep, right_id)) = self.insert_rec(child, key, val)? {
                    node.keys.insert(idx, sep);
                    node.children.insert(idx + 1, right_id);
                    if node.keys.len() > self.order {
                        let mid = node.keys.len() / 2;
                        let sep_up = node.keys[mid].clone();
                        let right_keys = node.keys.split_off(mid + 1);
                        let right_children = node.children.split_off(mid + 1);
                        let right_id2 = self.pager.allocate()?;
                        let right_node = Internal {
                            keys: right_keys,
                            children: right_children,
                        };
                        self.write_internal(node_id, &node)?; // left shrunk
                        self.write_internal(right_id2, &right_node)?;
                        return Ok(Some((sep_up, right_id2)));
                    } else {
                        self.write_internal(node_id, &node)?;
                        return Ok(None);
                    }
                } else {
                    self.write_internal(node_id, &node)?;
                    return Ok(None);
                }
            }
            _ => Ok(None),
        }
    }

    fn write_leaf(&mut self, id: PageId, leaf: &Leaf) -> io::Result<()> {
        let mut p = Page::new(id);
        p.set_magic();
        p.write_kind(PageKind::Leaf);
        let bytes = bincode::serialize(leaf).unwrap();
        if bytes.len() > PAGE_SIZE - 24 {
            return Err(io::Error::new(io::ErrorKind::Other, "leaf overflow"));
        }
        p.set_payload_len(bytes.len());
        p.payload_mut()[..bytes.len()].copy_from_slice(&bytes);
        self.pager.write_page(id, &p.buf)
    }

    fn write_internal(&mut self, id: PageId, node: &Internal) -> io::Result<()> {
        let mut p = Page::new(id);
        p.set_magic();
        p.write_kind(PageKind::Internal);
        let bytes = bincode::serialize(node).unwrap();
        if bytes.len() > PAGE_SIZE - 24 {
            return Err(io::Error::new(io::ErrorKind::Other, "internal overflow"));
        }
        p.set_payload_len(bytes.len());
        p.payload_mut()[..bytes.len()].copy_from_slice(&bytes);
        self.pager.write_page(id, &p.buf)
    }
}
