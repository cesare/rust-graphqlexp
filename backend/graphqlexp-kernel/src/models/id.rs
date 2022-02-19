use std::marker::PhantomData;

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
