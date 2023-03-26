#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BST<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<K: Ord, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> BST<K, V> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match self.root {
            Some(ref mut node) => node.insert(key, value),
            None => self.root = Some(Box::new(Node::new(key, value))),
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        match self.root {
            Some(ref node) => node.get(key).is_some(),
            None => false,
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.root {
            Some(ref node) => node.get(key),
            None => None,
        }
    }
}

impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }

    fn get(&self, needle: &K) -> Option<&V> {
        use std::cmp::Ordering;
        let mut curr = self;
        loop {
            match needle.cmp(&curr.key) {
                Ordering::Equal => return Some(&curr.value),
                Ordering::Less => match curr.left {
                    Some(ref left) => curr = left,
                    None => return None,
                },
                Ordering::Greater => match curr.right {
                    Some(ref right) => curr = right,
                    None => return None,
                },
            }
        }
    }

    fn insert(&mut self, key: K, value: V) {
        use std::cmp::Ordering;
        let mut curr = self;
        loop {
            match key.cmp(&curr.key) {
                Ordering::Equal => {
                    curr.value = value;
                    return;
                }
                Ordering::Less => match curr.left {
                    Some(ref mut left) => curr = left,
                    None => {
                        curr.left = Some(Box::new(Self::new(key, value)));
                        return;
                    }
                },
                Ordering::Greater => match curr.right {
                    Some(ref mut right) => curr = right,
                    None => {
                        curr.right = Some(Box::new(Self::new(key, value)));
                        return;
                    }
                },
            }
        }
    }
}

impl<K: Ord, V> Default for BST<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    #[test]
    fn test_fuzz() {
        use super::BST;
        use rand::prelude::*;
        use std::collections::BTreeMap;

        let mut rng = rand::thread_rng();
        let mut map = BTreeMap::new();
        let mut bst = BST::new();

        for _ in 0..1000 {
            let key = rng.gen::<u32>();
            let value = rng.gen::<u32>();
            map.insert(key, value);
            bst.insert(key, value);
        }

        for (key, value) in map.iter() {
            assert_eq!(bst.contains(key), true);
            assert_eq!(bst.contains(&(*key + 1)), false);
        }
    }

    // benchmark list lookup vs binary search tree lookup
    // of 10000 random numbers. (time must be lower on bst)
    #[test]
    fn bench_fuzz() {
        use super::BST;
        use rand::prelude::*;

        let mut rng = rand::thread_rng();
        let mut bst: BST<u32, Option<PhantomData<u32>>> = BST::new();
        let mut list = Vec::new();

        // just for fun
        let mut std_btset = std::collections::BTreeSet::new();
        let mut std_hashset = std::collections::HashSet::new();

        for _ in 0..10000 {
            let key = rng.gen::<u32>();
            bst.insert(key, None);
            list.push(key);
            std_btset.insert(key);
            std_hashset.insert(key);
        }

        let iter_list = list.clone();

        let time = std::time::Instant::now();

        for key in list.iter() {
            assert_eq!(bst.contains(key), true);
        }
        let bst_time = time.elapsed();
        println!("bst lookup took: {bst_time:?}");

        let time = std::time::Instant::now();

        for key in list.iter() {
            assert_eq!(std_btset.contains(key), true);
        }

        let std_btset = time.elapsed();
        println!("std_btset lookup took: {std_btset:?}");

        let time = std::time::Instant::now();

        for key in list.iter() {
            assert_eq!(std_hashset.contains(key), true);
        }

        let std_hashset = time.elapsed();
        println!("std_hashset lookup took: {std_hashset:?}");

        let time = std::time::Instant::now();

        for key in iter_list.iter() {
            assert_eq!(list.contains(key), true);
        }

        let list_time = time.elapsed();
        println!("list lookup took: {list_time:?}");

        assert!(list_time > bst_time);
    }
}
