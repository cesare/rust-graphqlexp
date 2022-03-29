use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Id<T, S> {
    pub value: S,
    _marker: PhantomData<T>,
}

impl<T, S> Id<T, S> {
    pub fn new(value: S) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T, S: Clone> Clone for Id<T, S> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<T, S: Hash> Hash for Id<T, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T, S: PartialEq> PartialEq for Id<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, S: Eq> Eq for Id<T, S> {}

pub trait Identifiable<T, S> {
    fn identifier(&self) -> &Id<T, S>;
}

pub trait BelongsTo<T, S> {
    fn parent_id(&self) -> &Id<T, S>;
}
