pub struct BPlusTree<K, V> {
    root: Option<Node<K, V>>,
    order: usize,
}

struct Node<K, V> {
    keys: Vec<K>,
    values: Vec<Option<V>>,
    children: Vec<Box<Node<K, V>>>,
    is_leaf: bool,
    next_leaf: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new_leaf() -> Self {
        Node {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf: true,
            next_leaf: None,
        }
    }

    fn new_internal() -> Self {
        Node {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf: false,
            next_leaf: None,
        }
    }
}

impl<K: Ord, V> BPlusTree<K, V> {
    pub fn new(order: usize) -> Self {
        assert!(order >= 2, "B+ tree order must be at least 2");
        BPlusTree {
            root: None,
            order,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tree() {
        let tree: BPlusTree<i32, String> = BPlusTree::new(3);
        assert!(tree.is_empty());
    }

    #[test]
    fn test_create_nodes() {
        let leaf = Node::<i32, String>::new_leaf();
        assert!(leaf.is_leaf);
        assert!(leaf.keys.is_empty());
        
        let internal = Node::<i32, String>::new_internal();
        assert!(!internal.is_leaf);
        assert!(internal.keys.is_empty());
    }

    #[test]
    #[should_panic(expected = "B+ tree order must be at least 2")]
    fn test_invalid_order() {
        let _tree: BPlusTree<i32, String> = BPlusTree::new(1);
    }
} 