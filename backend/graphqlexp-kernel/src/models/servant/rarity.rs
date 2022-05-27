use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Rarity(i32);

impl Rarity {
    const RARITY_RANGE: RangeInclusive<i32> = 0..=5;

    pub fn new(value: i32) -> Self {
        if ! Self::RARITY_RANGE.contains(&value) {
            panic!("Invalid rarity value: {}", value);
        }
        Self(value)
    }

    pub fn value(&self) -> i32 {
        self.0
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
        assert_eq!(Rarity(0), rarity0);

        let rarity5 = Rarity::new(5);
        assert_eq!(Rarity(5), rarity5);
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
