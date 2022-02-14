use anyhow::{Result, bail};

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
