use std::ops::RangeInclusive;

use anyhow::{Result, bail};

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
mod tests {
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
