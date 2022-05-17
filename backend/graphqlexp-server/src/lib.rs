pub mod config;
pub mod loaders;
pub mod logger;
pub mod routes;
pub mod schema;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphqlexpError {
    #[error("Initializing logger failed: {0}")]
    InitializingLoggerFailed(#[from] log::SetLoggerError),
}
