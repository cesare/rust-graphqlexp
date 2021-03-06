use chrono::{DateTime, Local};
use super::id::{Id, Identifiable};

mod rarity;
pub use rarity::Rarity;

mod class;
pub use class::Class;

mod name;
pub use name::Name;

pub type ServantId = Id<Servant>;

#[derive(Clone, Debug)]
pub struct Servant {
    pub id: ServantId,
    pub name: Name,
    pub class: Class,
    pub rarity: Rarity,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Identifiable<Servant> for Servant {
    fn identifier(&self) -> &ServantId {
        &self.id
    }
}
