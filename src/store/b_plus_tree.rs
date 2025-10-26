use std::{collections::HashMap, hash::Hash, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, sync::RwLock};

type NodeId = u64;

pub struct BPlusTreeStore {
    dir: PathBuf,
    wal: Mutex<File>,
    last_id: Arc<RwLock<u64>>,
    tree: RwLock<BPlusTree>,
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
