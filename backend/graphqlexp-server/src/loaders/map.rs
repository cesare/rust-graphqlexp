use std::collections::HashMap;
use std::marker::PhantomData;

use graphqlexp_app::models::{BelongsTo, Id, Identifiable};

pub struct OneToManyMap<T, S> {
    map: HashMap<Id<T>, Vec<S>>,
    _marker: PhantomData<T>,
}

impl<T, S> OneToManyMap<T, S> where T: Identifiable<T> + Clone, S: BelongsTo<T> + Clone {
    pub fn new(keys: &[Id<T>]) -> Self {
        let mut map = HashMap::with_capacity(keys.len());
        for key in keys.to_vec() {
            map.insert(key, vec![]);
        }

        Self {
            map,
            _marker: PhantomData,
        }
    }

    pub fn insert_all(&mut self, values: &[S]) {
        for v in values.to_vec() {
            let key = v.parent_id().to_owned();
            self.insert(&key, v);
        }
    }

    pub fn insert(&mut self, key: &Id<T>, value: S) {
        let values = self.map.get_mut(key).unwrap();
        values.push(value);
    }

    pub fn finish(self) -> HashMap<Id<T>, Vec<S>> {
        self.map
    }
}
