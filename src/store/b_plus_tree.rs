pub struct BPlusTree<K: Ord + Clone, V, const B: usize> {
    root: Node<K, V, B>,
}

enum Node<K: Ord + Clone, V, const B: usize> {
    Internal(Internal<K, V, B>),
    Leaf(Leaf<K, V, B>),
}

struct Internal<K: Ord + Clone, V, const B: usize> {
    keys: Vec<K>,.
    children: Vec<Box<Node<K, V, B>>>,
}

struct Leaf<K: Ord + Clone, V, const B: usize> {
    keys: Vec<K>,
    values: Vec<V>,
}
