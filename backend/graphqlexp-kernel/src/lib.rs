pub mod models;
pub mod repositories;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid position {0}")]
    InvalidPosition(i32),
}
