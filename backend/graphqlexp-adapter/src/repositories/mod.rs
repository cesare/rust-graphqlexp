use std::marker::PhantomData;

use crate::persistence::Database;

pub mod servant;

pub struct Repository<T> {
    #[allow(dead_code)]
    database: Database,
    _marker: PhantomData<T>,
}

impl<T> Repository<T> {
    pub fn new(database: Database) -> Self {
        Self {
            database: database,
            _marker: PhantomData,
        }
    }
}
