use std::ops::RangeInclusive;

use crate::models::GraphqlexpError;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Rarity {
    value: i32,
}

impl Rarity {
    const RARITY_RANGE: RangeInclusive<i32> = 0..=5;

    pub fn new(value: i32) -> Self {
        value.try_into().unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl TryFrom<i32> for Rarity {
    type Error = GraphqlexpError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if ! Self::RARITY_RANGE.contains(&value) {
            return Err(GraphqlexpError::InvalidRarity(value))
        }
        Ok(Self { value })
    }
}

#[cfg(test)]
mod tests {
    use super::Rarity;

    #[test]
    #[should_panic]
    fn creation_with_negative_values() {
        let _negative_rarity = Rarity::new(-1);
    }

    #[test]
    #[should_panic]
    fn creation_with_excessive_values() {
        let _exceeding_rarity = Rarity::new(6);
    }

    #[test]
    fn creation_with_valid_values() {
        let rarity0 = Rarity::new(0);
        assert_eq!(Rarity { value: 0 }, rarity0);

        let rarity5 = Rarity::new(5);
        assert_eq!(Rarity { value: 5 }, rarity5);
    }

    #[test]
    fn raw_value() {
        assert_eq!(3, Rarity::new(3).value());
    }

    #[test]
    fn ordering_and_equality() {
        assert!(Rarity::new(4) == Rarity::new(4));
        assert!(Rarity::new(4) <= Rarity::new(4));
        assert!(Rarity::new(4) <  Rarity::new(5));
    }
}
