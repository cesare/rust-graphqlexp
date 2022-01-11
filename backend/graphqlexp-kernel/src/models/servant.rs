use std::str::FromStr;

use anyhow::anyhow;
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

impl FromStr for Class {
    type Err = anyhow::Error;

    fn from_str(class_name: &str) -> Result<Self, Self::Err> {
        match class_name {
            "saber" => Ok(Self::Saber),
            "archer" => Ok(Self::Archer),
            "lancer" => Ok(Self::Lancer),
            "rider" => Ok(Self::Rider),
            "caster" => Ok(Self::Caster),
            "assassin" => Ok(Self::Assassin),
            "berserker" => Ok(Self::Berserker),
            "ruler" => Ok(Self::Ruler),
            "avenger" => Ok(Self::Avenger),
            "mooncancer" => Ok(Self::Mooncancer),
            "alterago" => Ok(Self::Alterego),
            "pretender" => Ok(Self::Pretender),
            "foreigner" => Ok(Self::Foreigner),
            _ => Err(anyhow!("Unknown servant class name: {}", class_name)),
        }
    }
}

pub struct Servant {
    pub id: ServantId,
    pub name: String,
    pub class: Class,
    pub rarity: u32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
