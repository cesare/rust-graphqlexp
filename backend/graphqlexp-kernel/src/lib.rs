pub mod models;
pub mod repositories;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown servant class {0}")]
    UnknownClass(String),
}
