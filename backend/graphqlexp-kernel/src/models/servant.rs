use chrono::{DateTime, Local};
use super::id::Id;

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
    pub id: Id<Servant, u32>,
    pub name: String,
    pub class: Class,
    pub rarity: u32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
