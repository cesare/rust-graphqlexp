use anyhow::{Result, bail};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ProfilePosition(i32);

impl ProfilePosition {
    pub fn create(value: i32) -> Result<Self> {
        if value < 1 {
            bail!("invalid profile position: {}", value);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ProfilePosition;

    #[test]
    fn creation_with_valid_values() {
        let position_1 = ProfilePosition::create(1);
        assert!(position_1.is_ok());
        assert_eq!(ProfilePosition(1), position_1.unwrap());

        let position_123 = ProfilePosition::create(123);
        assert!(position_123.is_ok());
        assert_eq!(ProfilePosition(123), position_123.unwrap());
    }

    #[test]
    fn creation_with_zero() {
        let position = ProfilePosition::create(0);
        assert!(position.is_err());
    }

    #[test]
    fn creation_with_negative_values() {
        let position = ProfilePosition::create(-1);
        assert!(position.is_err());
    }

    #[test]
    fn ordering_and_equality() {
        assert!(ProfilePosition(3) == ProfilePosition(3));
        assert!(ProfilePosition(3) <= ProfilePosition(3));
        assert!(ProfilePosition(1) <= ProfilePosition(2));
        assert!(ProfilePosition(1) <  ProfilePosition(2));
    }

    #[test]
    fn values() {
        assert_eq!(1, ProfilePosition(1).value());
    }
}
