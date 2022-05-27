#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ProfilePosition {
    value: i32,
}

impl ProfilePosition {
    pub fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Invalid profile position value: {}", value);
        }
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::ProfilePosition;

    #[test]
    fn creation_with_valid_values() {
        let position_1 = ProfilePosition::new(1);
        assert_eq!(ProfilePosition { value: 1 }, position_1);

        let position_123 = ProfilePosition::new(123);
        assert_eq!(ProfilePosition { value: 123 }, position_123);
    }

    #[test]
    #[should_panic]
    fn creation_with_zero() {
        let _position = ProfilePosition::new(0);
    }

    #[test]
    #[should_panic]
    fn creation_with_negative_values() {
        let _position = ProfilePosition::new(-1);
    }

    #[test]
    fn ordering_and_equality() {
        assert!(ProfilePosition::new(3) == ProfilePosition::new(3));
        assert!(ProfilePosition::new(3) <= ProfilePosition::new(3));
        assert!(ProfilePosition::new(1) <= ProfilePosition::new(2));
        assert!(ProfilePosition::new(1) <  ProfilePosition::new(2));
    }

    #[test]
    fn values() {
        assert_eq!(1, ProfilePosition::new(1).value());
    }
}
