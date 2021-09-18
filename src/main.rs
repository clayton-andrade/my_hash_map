use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_BUCKET_SIZE: usize = 5381;

#[derive(Debug)]
struct MyMap<K, V> {
    buckets: Vec<Option<(K, V)>>,
}

impl<K, V> MyMap<K, V> {
    fn new() -> Self {
        let mut buckets: Vec<Option<(K, V)>> = Vec::with_capacity(INITIAL_BUCKET_SIZE);
        for _ in 0..INITIAL_BUCKET_SIZE {
            buckets.push(None);
        }
        MyMap { buckets }
    }

    fn insert(&mut self, key: K, value: V)
    where
        K: Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let index = hasher.finish() % self.buckets.len() as u64;
        self.buckets[index as usize] = Some((key, value));
        // match self.buckets.iter().enumerate().find(|(_, (k, _))| *k == key).map(|(i, (_, _))| i) {
        //     Some(i) => {
        //         let (_, v) = self.buckets.get_mut(i).unwrap();
        //         *v = value;
        //     },
        //     None => self.buckets.push((key, value)),
        // }
    }

    // fn get(&self, key: &K) -> Option<&V>
    // where
    //     K: std::cmp::Eq,
    // {
    //     self.buckets.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    // }

    // fn remove(&mut self, key: K) -> Option<V>
    // where
    //     K: std::cmp::Eq,
    //     V: Copy + Clone,
    // {
    //     let value;
    //     let index;
    //     if let Some(i) = self.buckets.iter().enumerate().find(|(_, (k, _))| *k == key).map(|(i, (_, _))| i) {
    //         let (_, v) = self.buckets.get(i).unwrap();
    //         value = v.to_owned();
    //         index = i;
    //     } else {
    //         return None;
    //     }
    //     self.buckets.remove(index);
    //     Some(value)
    // }
}

fn main() {
    let mut map = MyMap::new();
    map.insert("maçã", 45);
    println!("{:?}", map);
    map.insert("maçã", 56);
    println!("{:?}", map);
    map.insert("pera", 22);
    // assert_eq!(map.get(&"limão"), None);
    // assert_eq!(map.get(&"pera"), Some(&22));
    // println!("{:?}", map);
    // assert_eq!(map.remove("maçã"), Some(56));
    // println!("{:?}", map);
    // assert_eq!(map.remove("limão"), None);
    println!("{:?}", map);
}
