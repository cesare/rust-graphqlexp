use std::collections::HashMap;
use std::hash::Hash;

pub(super) struct HasManyMap<K, T> {
    map: HashMap<K, Vec<T>>,
}

impl<K, T> HasManyMap<K, T> where K: Clone + Eq + Hash {
    pub fn new(keys: &[K]) -> Self {
        let mut map = HashMap::with_capacity(keys.len());
        for key in keys {
            map.insert(key.clone(), vec![]);
        }

        Self { map }
    }

    pub fn insert(&mut self, key: &K, value: T) {
        let values = self.map.get_mut(key).unwrap();
        values.push(value);
    }

    pub fn finish(self) -> HashMap<K, Vec<T>> {
        self.map
    }
}
