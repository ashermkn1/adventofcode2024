use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Wrapper<K: Ord, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> Eq for Wrapper<K, V> {}

impl<K: Ord, V> PartialEq<Self> for Wrapper<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> PartialOrd<Self> for Wrapper<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Wrapper<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key)
    }
}

#[derive(Default)]
pub struct MinHeap<K: Ord, V> {
    heap: BinaryHeap<Wrapper<K, V>>,
}

impl<K: Ord, V> MinHeap<K, V> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        MinHeap {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(Wrapper { key, value })
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|w| (w.key, w.value))
    }

    #[allow(dead_code)]
    pub fn peek(&self) -> Option<(&K, &V)> {
        self.heap.peek().map(|w| (&w.key, &w.value))
    }
}
