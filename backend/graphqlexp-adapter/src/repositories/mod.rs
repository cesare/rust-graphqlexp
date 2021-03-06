use std::marker::PhantomData;

use crate::persistence::Database;

pub mod profile;
pub mod servant;

pub struct Repository<T> {
    database: Database,
    _marker: PhantomData<T>,
}

impl<T> Repository<T> {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            _marker: PhantomData,
        }
    }
}
