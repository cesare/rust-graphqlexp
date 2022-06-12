use thiserror::Error;

pub mod id;
pub mod profile;
pub mod servant;

#[derive(Error, Debug)]
pub enum GraphqlexpError {
    #[error("Unknown servant class: {0}")]
    UnknownClass(String),

    #[error("Invalid servant name: {0}")]
    InvalidServantName(String),

    #[error("Invalid rarity value: {0}")]
    InvalidRarity(i32),

    #[error("Invalid profile position value: {0}")]
    InvalidProfilePosition(i32),
}
