use std::convert::From;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Id<T> {
    pub value: String,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: String) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T> From<String> for Id<T> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> {}

pub trait Identifiable<T> {
    fn identifier(&self) -> &Id<T>;
}

pub trait BelongsTo<T> {
    fn parent_id(&self) -> &Id<T>;
}
