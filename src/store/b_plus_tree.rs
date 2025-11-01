use std::{
    collections::HashMap,
    hash::Hash,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

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
        if let Some(v) = self.tree.read().get(&key) {}

        Ok(None)
    }

    async fn put(&self, record: LinkRecord) -> anyhow::Result<()> {
        // Implementation goes here
        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone)]
struct BpTree {
    order: usize,
    root: NodeId,
    next_id: NodeId,
    nodes: std::collections::HashMap<NodeId, Node>,
}

impl BpTree {
    fn new(order: usize) -> Self {
        let root = 0;
        let mut nodes = HashMap::new();
        nodes.insert(
            root,
            Node::Leaf {
                keys: Vec::new(),
                vals: Vec::new(),
                next: None,
            },
        );
        Self {
            order: order.max(4),
            root,
            next_id: 1,
            nodes,
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let mut current_node_id = self.root;
        loop {
            let node = self.nodes.get(&current_node_id)?;
            match node {
                Node::Internal { keys, children } => {
                    let idx = match keys.binary_search_by(|k| k.as_slice().cmp(key)) {
                        Ok(i) => i + 1,
                        Err(i) => i,
                    };
                    let i = idx.min(children.len() - 1);
                    current_node_id = children[i];
                }
                Node::Leaf { keys, vals, .. } => {
                    if let Ok(i) = keys.binary_search_by(|k| k.as_slice().cmp(key)) {
                        let v = vals[i].clone();
                        return if v.is_empty() { None } else { Some(v) };
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    fn insert(&mut self, key: Vec<u8>, val: Vec<u8>) {
        // Implementation goes here
    }

    fn alloc_leaf(
        &mut self,
        keys: Vec<Vec<u8>>,
        vals: Vec<Vec<u8>>,
        next: Option<NodeId>,
    ) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, Node::Leaf { keys, vals, next });
        id
    }
    fn alloc_internal(&mut self, keys: Vec<Vec<u8>>, children: Vec<NodeId>) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, Node::Internal { keys, children });
        id
    }
}
