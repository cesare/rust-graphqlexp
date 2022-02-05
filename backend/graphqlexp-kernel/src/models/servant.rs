use std::ops::RangeInclusive;
use std::str::FromStr;
use std::string::ToString;

use anyhow::{Result, anyhow, bail};
use chrono::{DateTime, Local};
use super::id::Id;

pub type ServantId = Id<Servant, i32>;

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
            "alterego" => Ok(Self::Alterego),
            "pretender" => Ok(Self::Pretender),
            "foreigner" => Ok(Self::Foreigner),
            _ => Err(anyhow!("Unknown servant class name: {}", class_name)),
        }
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        let class_name = match self {
            Self::Saber => "saber",
            Self::Archer => "archer",
            Self::Lancer => "lancer",
            Self::Rider => "rider",
            Self::Caster => "caster",
            Self::Assassin => "assassin",
            Self::Berserker => "berserker",
            Self::Ruler => "ruler",
            Self::Avenger => "avenger",
            Self::Mooncancer => "mooncancer",
            Self::Alterego => "alterego",
            Self::Pretender => "pretender",
            Self::Foreigner => "foreigner",
        };
        class_name.to_owned()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Rarity(i32);

impl Rarity {
    const RARITY_RANGE: RangeInclusive<i32> = 0..=5;

    pub fn create(value: i32) -> Result<Self> {
        if ! Self::RARITY_RANGE.contains(&value) {
            bail!("Invalid rarity value: {}", value);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::Rarity;

    #[test]
    fn creation_with_invalid_values() {
        let negative_rarity = Rarity::create(-1);
        assert!(negative_rarity.is_err(), "creating rarity with negative values should fail");

        let exceeding_rarity = Rarity::create(6);
        assert!(exceeding_rarity.is_err(), "creating rarity with values over upper limit should fail");
    }

    #[test]
    fn creation_with_valid_values() {
        let rarity0 = Rarity::create(0);
        assert!(rarity0.is_ok());
        assert_eq!(Rarity(0), rarity0.unwrap());

        let rarity5 = Rarity::create(5);
        assert!(rarity5.is_ok());
        assert_eq!(Rarity(5), rarity5.unwrap());
    }

    #[test]
    fn raw_value() {
        assert_eq!(3, Rarity(3).value());
    }

    #[test]
    fn ordering_and_equality() {
        assert!(Rarity(4) == Rarity(4));
        assert!(Rarity(4) <= Rarity(4));
        assert!(Rarity(4) <  Rarity(5));
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
