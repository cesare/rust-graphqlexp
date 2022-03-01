use std::convert::From;

use chrono::{DateTime, Local};
use super::id::Id;

mod rarity;
pub use rarity::Rarity;

mod class;
pub use class::Class;

pub type ServantId = Id<Servant, i32>;

impl From<i32> for ServantId {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

pub struct Servant {
    pub id: ServantId,
    pub name: String,
    pub class: Class,
    pub rarity: Rarity,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
