use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

#[allow(dead_code)]
struct Node<T: Eq + Copy> {
    value: T,
    parent: T,
    size: usize,
}

impl<T: Eq + Copy> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            parent: value,
            size: 1,
        }
    }
}

pub struct UnionFind<T: Eq + Copy + Hash> {
    forest: HashMap<T, Node<T>>,
    num_trees: usize,
}

impl<T: Eq + Copy + Hash> UnionFind<T> {
    #[allow(dead_code)]
    pub fn new(keys: &[T]) -> Self {
        let mut forest = HashMap::new();
        for &k in keys {
            forest.insert(k, Node::new(k));
        }

        Self {
            forest,
            num_trees: keys.len(),
        }
    }

    pub fn from_iter(keys: impl Iterator<Item = T>) -> Self {
        let mut forest = HashMap::new();
        let mut num_trees = 0;
        for k in keys {
            forest.insert(k, Node::new(k));
            num_trees += 1;
        }

        Self { forest, num_trees }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, key: T) {
        if let Entry::Vacant(e) = self.forest.entry(key) {
            e.insert(Node::new(key));
            self.num_trees += 1;
        }
    }

    pub fn find(&mut self, key: T) -> T {
        let mut curr = key;
        while self.forest[&curr].parent != curr {
            self.forest.get_mut(&curr).unwrap().parent =
                self.forest[&self.forest[&curr].parent].parent;
            curr = self.forest[&curr].parent;
        }
        curr
    }

    pub fn union(&mut self, k1: T, k2: T) {
        let x = self.find(k1);
        let y = self.find(k2);

        if x != y {
            let (x, y) = if self.forest[&x].size < self.forest[&y].size {
                (y, x)
            } else {
                (x, y)
            };

            self.forest.get_mut(&y).unwrap().parent = x;
            self.forest.get_mut(&x).unwrap().size += self.forest[&y].size;
            self.num_trees -= 1;
        }
    }
}
