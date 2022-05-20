pub mod models;
pub mod modules;
pub mod persistence;
pub mod records;
pub mod repositories;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("query failed")]
    QueryFailure(#[from] sqlx::Error),

    #[error("failed to generate cuid")]
    CuidFailure(#[from] cuid::CuidError),

    #[error(transparent)]
    KernelError(#[from] graphqlexp_kernel::Error),
}
