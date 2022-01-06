use chrono::{DateTime, Local};
use super::id::Id;

pub type ServantId = Id<Servant, u32>;

pub enum Class {
    Saber,
    Archer,
    Lancer,
    Rider,
    Caster,
    Assassin,
    Berserker,
    Ruler,
    Avenger,
    Mooncancer,
    Alterego,
    Pretender,
    Foreigner,
}

pub struct Servant {
    pub id: ServantId,
    pub name: String,
    pub class: Class,
    pub rarity: u32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
