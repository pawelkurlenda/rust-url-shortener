use std::{collections::HashMap, hash::Hash, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, sync::RwLock};

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

pub struct BPlusTree {
    root: NodeId,
    nodes: HashMap<NodeId, Node>,
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
