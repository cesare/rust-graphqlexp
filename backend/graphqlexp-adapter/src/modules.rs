use graphqlexp_kernel::{
    models::servant::Servant,
};
use crate::{
    persistence::Database,
    repositories::Repository,
};

pub struct RepositoriesModule {
    servant_repository: Repository<Servant>,
}

impl RepositoriesModule {
    pub fn new(db: Database) -> Self {
        Self {
            servant_repository: Repository::new(db.clone()),
        }
    }
}

pub trait RepositoriesModuleExt {
    fn servant_repository(&self) -> &Repository<Servant>;
}

impl RepositoriesModuleExt for RepositoriesModule {
    fn servant_repository(&self) -> &Repository<Servant> {
        &self.servant_repository
    }
}
