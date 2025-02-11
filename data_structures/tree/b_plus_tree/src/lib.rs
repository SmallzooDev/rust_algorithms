use std::rc::Rc;
use std::cell::RefCell;

pub struct BPlusTree<K, V> {
    root: Option<Box<Node<K, V>>>,
    order: usize,
}

struct Node<K, V> {
    keys: Vec<K>,
    values: Vec<Option<V>>,
    children: Vec<Box<Node<K, V>>>,
    is_leaf: bool,
    next_leaf: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K: Ord, V> Node<K, V> {
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

    fn search(&self, key: &K) -> Option<&V> {
        if self.is_leaf {
            match self.keys.binary_search(key) {
                Ok(idx) => self.values[idx].as_ref(),
                Err(_) => None,
            }
        } else {
            let child_idx = match self.keys.binary_search(key) {
                Ok(idx) => idx + 1,
                Err(idx) => idx,
            };
            if child_idx < self.children.len() {
                self.children[child_idx].search(key)
            } else {
                None
            }
        }
    }
}

impl<K: Ord, V> BPlusTree<K, V> {
    pub fn new(order: usize) -> Self {
        assert!(order >= 2, "B+ tree order must be at least 2");
        BPlusTree { root: None, order }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        match &self.root {
            Some(root) => root.search(key),
            None => None,
        }
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

    #[test]
    fn test_search_empty_tree() {
        let tree: BPlusTree<i32, String> = BPlusTree::new(3);
        assert_eq!(tree.search(&1), None);
    }

    #[test]
    fn test_search_single_node() {
        let mut tree = BPlusTree::new(3);
        let root = Node {
            keys: vec![1, 2],
            values: vec![Some("one".to_string()), Some("two".to_string())],
            children: vec![],
            is_leaf: true,
            next_leaf: None,
        };
        tree.root = Some(Box::new(root));

        assert_eq!(tree.search(&1), Some(&"one".to_string()));
        assert_eq!(tree.search(&2), Some(&"two".to_string()));
        assert_eq!(tree.search(&3), None);
    }
}

