use std::{collections::HashMap, hash::Hash, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, sync::RwLock};

use crate::{models::LinkRecord, store::store::Store};

const WAL_FILE: &str = "wal.log";
type NodeId = u64;

pub struct BPlusTreeStore {
    dir: PathBuf,
    wal: Mutex<File>,
    last_id: Arc<RwLock<u64>>,
    tree: RwLock<BPlusTree>,
}

impl BPlusTreeStore {
    pub async fn new(dir: impl AsRef<Path>) -> tokio::io::Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        tokio::fs::create_dir_all(&dir).await?;
        let wal_path = dir.join(WAL_FILE);
        let wal = File::options()
            .create(true)
            .append(true)
            .open(wal_path)
            .await?;
        let tree = BPlusTree {
            root: 0,
            nodes: HashMap::new(),
        };
        Ok(Self {
            dir,
            wal: Mutex::new(wal),
            last_id: Arc::new(RwLock::new(0)),
            tree: RwLock::new(tree),
        })
    }
}

#[async_trait]
impl Store for BPlusTreeStore {
    async fn get(&self, id: &str) -> anyhow::Result<Option<LinkRecord>> {
        let key = ns_link_v1(id);
        if let Some(v) = self.tree.read().get(&key) {
            
        }

        Ok(None)
    }

    async fn put(&self, record: LinkRecord) -> anyhow::Result<()> {
        // Implementation goes here
        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()>{
        // Implementation goes here
        Ok(())
    }

    async fn incr_hit(&self, id: &str) -> anyhow::Result<()> {
        // Implementation goes here
        Ok(())
    }

    async fn get_hits(&self, id: &str) -> anyhow::Result<u64> {
        // Implementation goes here
        Ok(0)
    }
}

fn ns_link_v1(id: &str) -> Vec<u8> {
    let mut v = b"L:".to_vec();
    v.extend_from_slice(id.as_bytes());
    v
}

fn ns_link_v2(id: &str) -> Vec<u8> {
    let mut v = Vec::with_capacity(2 + id.len());
    v.extend_from_slice(b"L:");
    v.extend_from_slice(id.as_bytes());
    v
}

#[derive(Serialize, Deserialize, Clone)]
struct BpTree {
    order: usize,
    root: NodeId,
    next_id: NodeId,
    nodes: std::collections::HashMap<NodeId, Node>,
}

#[derive(Serialize, Deserialize, Clone)]
enum Node {
    Internal {
        keys: Vec<Vec<u8>>,
        children: Vec<u64>,
    },
    Leaf {
        keys: Vec<Vec<u8>>,
        vals: Vec<Vec<u8>>,
        next: Option<u64>,
    },
}

struct Internal<K: Ord + Clone, V, const B: usize> {
    keys: Vec<K>,.
    children: Vec<Box<Node<K, V, B>>>,
}

struct Leaf<K: Ord + Clone, V, const B: usize> {
    keys: Vec<K>,
    values: Vec<V>,
}
