pub mod models;
pub mod repositories;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown servant class {0}")]
    UnknownClass(String),

    #[error("invalid rarity {0}")]
    InvalidRarity(i32),

    #[error("invalid position {0}")]
    InvalidPosition(i32),
}
