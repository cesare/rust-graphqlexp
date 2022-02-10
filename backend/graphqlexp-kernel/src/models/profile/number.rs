use anyhow::{Result, bail};

pub struct ProfileNumber(i32);

impl ProfileNumber {
    pub fn create(value: i32) -> Result<Self> {
        if value < 1 {
            bail!("invalid profile number: {}", value);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
