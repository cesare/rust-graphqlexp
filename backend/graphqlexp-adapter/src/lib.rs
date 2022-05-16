pub mod models;
pub mod modules;
pub mod persistence;
pub mod records;
pub mod repositories;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    KernelError(#[from] graphqlexp_kernel::Error),
}
