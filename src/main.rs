use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_BUCKET_SIZE: usize = 5381;

#[derive(Debug)]
struct MyMap<K, V> {
    buckets: Vec<Option<(K, V)>>,
}

impl<K: Hash, V: Clone> MyMap<K, V> {
    fn new() -> Self {
        let mut buckets: Vec<Option<(K, V)>> = Vec::with_capacity(INITIAL_BUCKET_SIZE);
        for _ in 0..INITIAL_BUCKET_SIZE {
            buckets.push(None);
        }
        MyMap { buckets }
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        self.buckets[index] = Some((key, value));
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        self.buckets.get(index).and_then(|elem| elem.as_ref()).map(|(_, v)| v)
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let index = self.hash(&key);
        if let Some(v) = self.buckets.get(index).and_then(|elem| elem.as_ref()).map(|(_, v)| v.clone()) {
              self.buckets.remove(index);
              Some(v)
        } else {
            return None;
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }
}

fn main() {
    let mut map = MyMap::new();
    map.insert("maçã", 45);
    map.insert("maçã", 56);
    map.insert("pera", 22);
    assert_eq!(map.get(&"limão"), None);
    assert_eq!(map.get(&"pera"), Some(&22));
    assert_eq!(map.remove("maçã"), Some(56));
    assert_eq!(map.remove("limão"), None);
    for pair in &map.buckets {
        if pair.is_none() {
            continue;
        }
        println!("{}: {}", pair.as_ref().unwrap().0, pair.as_ref().unwrap().1);
    }
    
}
