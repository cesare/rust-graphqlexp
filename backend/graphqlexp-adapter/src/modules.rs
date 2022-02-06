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
    database: Database,
}

impl RepositoriesModule {
    pub fn new(db: Database) -> Self {
        Self {
            database: db,
        }
    }

    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self> {
        let database = Database::create(config).await?;
        Ok(Self::new(database))
    }

    pub fn servant_repository(&self) -> Repository<Servant> {
        Repository::new(self.database.clone())
    }
}
