use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use async_trait::async_trait;

use crate::{
    models::LinkRecord,
    store::{
        bptree::{
            pager::Pager,
            tree::BPlusTree,
            wal::{Operation, Wal},
        },
        store::Store,
    },
};

fn ns_link(id: &str) -> Vec<u8> {
    let mut v = b"L:".to_vec();
    v.extend_from_slice(id.as_bytes());
    v
}

fn ns_hit(id: &str) -> Vec<u8> {
    let mut v = b"H:".to_vec();
    v.extend_from_slice(id.as_bytes());
    v
}

pub struct BpStore {
    dir: PathBuf,
    wal: Mutex<Wal>,
    tree: Mutex<BPlusTree>,
}

impl BpStore {
    pub async fn open(dir: impl AsRef<Path>, order: usize) -> Result<Self, std::io::Error> {
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir)?;

        let data_path = dir.join("data.dat");
        let wal_path = dir.join("wal.log");

        let pager = Pager::open(&data_path)?;
        let mut tree = BPlusTree::open(pager, order)?;
        let mut wal = Wal::open(&wal_path)?;

        wal.replay(|_lsn, op, key, val| match op {
            Operation::Insert => {
                let _ = tree.put(key, val);
            }
            Operation::Delete => {
                let _ = tree.put(key, Vec::new());
            }
        })?;

        Ok(Self {
            dir,
            wal: Mutex::new(wal),
            tree: Mutex::new(tree),
        })
    }
}

#[async_trait]
impl Store for BpStore {
    async fn get(&self, id: &str) -> anyhow::Result<Option<LinkRecord>> {
        let key = ns_link(id);
        let mut tree = self.tree.lock();
        if let Some(v) = tree.get(&key)? {
            if v.is_empty() {
                return Ok(None);
            }
            let rec: LinkRecord = bincode::deserialize(&v)?;
            let now = time::OffsetDateTime::now_utc();
            if rec.is_expired(now) {
                return Ok(None);
            }
            Ok(Some(rec))
        } else {
            Ok(None)
        }
    }

    async fn put(&self, record: LinkRecord) -> anyhow::Result<()> {
        let key = ns_link(&record.id);
        let val = bincode::serialize(&record)?;
        {
            let mut wal = self.wal.lock();
            let _lsn = wal.append(Operation::Insert, &key, &val)?;
        }
        self.tree.lock().put(key, val)?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<bool> {
        let key = ns_link(id);
        {
            let mut wal = self.wal.lock();
            let _lsn = wal.append(Operation::Delete, &key, &[])?;
        }
        self.tree.lock().put(key, Vec::new())?;
        Ok(true)
    }

    async fn incr_hit(&self, id: &str) -> anyhow::Result<()> {
        let key = ns_hit(id);
        let cur = self
            .tree
            .lock()
            .get(&key)?
            .map(|v| {
                if v.is_empty() {
                    0
                } else {
                    u64::from_le_bytes(v[..8].try_into().unwrap())
                }
            })
            .unwrap_or(0);
        let next = (cur + 1).to_le_bytes().to_vec();
        {
            let mut wal = self.wal.lock();
            let _lsn = wal.append(Operation::Insert, &key, &next)?;
        }
        self.tree.lock().put(key, next)?;
        Ok(())
    }

    async fn get_hits(&self, id: &str) -> anyhow::Result<u64> {
        let key = ns_hit(id);
        if let Some(v) = self.tree.lock().get(&key)? {
            if v.is_empty() {
                Ok(0)
            } else {
                Ok(u64::from_le_bytes(v[..8].try_into().unwrap()))
            }
        } else {
            Ok(0)
        }
    }
}
