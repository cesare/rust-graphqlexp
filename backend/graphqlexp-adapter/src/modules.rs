use anyhow::Result;

use graphqlexp_kernel::{
    models::servant::Servant,
};
use crate::{
    persistence::Database,
    repositories::Repository,
};

pub trait RepositoriesModuleConfig {
    fn database_url(&self) -> String;
}

pub struct RepositoriesModule {
    servant_repository: Repository<Servant>,
}

impl RepositoriesModule {
    pub fn new(db: Database) -> Self {
        Self {
            servant_repository: Repository::new(db.clone()),
        }
    }

    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self> {
        let database = Database::create(config).await?;
        Ok(Self::new(database))
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
