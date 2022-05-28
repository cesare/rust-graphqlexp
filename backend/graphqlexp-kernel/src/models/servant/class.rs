use std::string::ToString;

#[derive(Clone, Debug, PartialEq)]
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

impl From<&str> for Class {
    fn from(class_name: &str) -> Self {
        match class_name {
            "saber"      => Self::Saber,
            "archer"     => Self::Archer,
            "lancer"     => Self::Lancer,
            "rider"      => Self::Rider,
            "caster"     => Self::Caster,
            "assassin"   => Self::Assassin,
            "berserker"  => Self::Berserker,
            "ruler"      => Self::Ruler,
            "avenger"    => Self::Avenger,
            "mooncancer" => Self::Mooncancer,
            "alterego"   => Self::Alterego,
            "pretender"  => Self::Pretender,
            "foreigner"  => Self::Foreigner,
            _ => panic!("Unknown class name: {}", class_name)
        }
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        let class_name = match self {
            Self::Saber      => "saber",
            Self::Archer     => "archer",
            Self::Lancer     => "lancer",
            Self::Rider      => "rider",
            Self::Caster     => "caster",
            Self::Assassin   => "assassin",
            Self::Berserker  => "berserker",
            Self::Ruler      => "ruler",
            Self::Avenger    => "avenger",
            Self::Mooncancer => "mooncancer",
            Self::Alterego   => "alterego",
            Self::Pretender  => "pretender",
            Self::Foreigner  => "foreigner",
        };
        class_name.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Class;

    #[test]
    fn to_string() {
        assert_eq!("saber", Class::Saber.to_string());
        assert_eq!("alterego", Class::Alterego.to_string());
    }

    #[test]
    fn valid_string_to_class() {
        let saber = Class::from("saber");
        assert_eq!(Class::Saber, saber);

        let foreigner = Class::from("foreigner");
        assert_eq!(Class::Foreigner, foreigner);
    }

    #[test]
    #[should_panic]
    fn invalid_string_to_class() {
        let _ = Class::from("faker");
    }

    #[test]
    fn equality() {
        assert!(Class::Saber == Class::Saber);
        assert!(Class::Saber != Class::Avenger);
    }
}
