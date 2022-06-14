use std::str::FromStr;

use crate::models::GraphqlexpError;

#[derive(Clone, Debug)]
pub struct ProfileText {
    value: String,
}

impl ProfileText {
    pub fn new(value: &str) -> Self {
        Self::from_str(value).unwrap_or_else(|e| panic!("{}", e))
    }
}

impl AsRef<str> for ProfileText {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl FromStr for ProfileText {
    type Err = GraphqlexpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized_value = s.trim();
        if normalized_value.is_empty() {
            return Err(GraphqlexpError::InvalidProfileText(s.to_owned()))
        }

        Ok(Self { value: normalized_value.to_owned() })
    }
}
