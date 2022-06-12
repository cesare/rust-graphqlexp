use std::str::FromStr;

use crate::models::GraphqlexpError;

#[derive(Clone, Debug)]
pub struct Name {
    value: String,
}

impl Name {
    pub fn new(value: &str) -> Self {
        Self::from_str(value).unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl FromStr for Name {
    type Err = GraphqlexpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized_name = s.trim();
        if normalized_name.is_empty() {
            return Err(GraphqlexpError::InvalidServantName(s.to_owned()))
        }

        Ok(Self { value: normalized_name.to_owned() })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Name;

    #[test]
    fn creation_with_valid_name() {
        let name = Name::from_str("Meltryllis");
        assert!(name.is_ok());
        assert_eq!("Meltryllis", name.unwrap().value());
    }

    #[test]
    fn creation_with_normalized_name() {
        let name = Name::from_str(" Morgan \n");
        assert!(name.is_ok());
        assert_eq!("Morgan", name.unwrap().value());
    }

    #[test]
    fn creation_with_invalid_name() {
        assert!(Name::from_str("").is_err());
        assert!(Name::from_str("  \n  \r\n").is_err());
    }
}
