use std::str::FromStr;
use std::string::ToString;

use anyhow::anyhow;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Class;

    #[test]
    fn to_string() {
        assert_eq!("saber", Class::Saber.to_string());
        assert_eq!("alterego", Class::Alterego.to_string());
    }

    #[test]
    fn valid_string_to_class() {
        let saber = Class::from_str("saber");
        assert!(saber.is_ok());
        assert_eq!(Class::Saber, saber.unwrap());

        let foreigner = Class::from_str("foreigner");
        assert!(foreigner.is_ok());
        assert_eq!(Class::Foreigner, foreigner.unwrap());
    }

    #[test]
    fn invalid_string_to_class() {
        assert!(Class::from_str("faker").is_err());
    }

    #[test]
    fn equality() {
        assert!(Class::Saber == Class::Saber);
        assert!(Class::Saber != Class::Avenger);
    }
}
