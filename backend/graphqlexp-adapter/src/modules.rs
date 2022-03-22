use anyhow::Result;

use graphqlexp_kernel::{
    models::{
        profile::Profile,
        servant::Servant,
    },
};
use crate::{
    persistence::Database,
    repositories::Repository,
};

pub trait RepositoriesModuleConfig {
    fn database_url(&self) -> String;
}

#[derive(Clone)]
pub struct RepositoriesModule {
    database: Database,
}

impl RepositoriesModule {
    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self> {
        let database = Database::create(config).await?;
        Ok(Self { database })
    }

    pub fn servant_repository(&self) -> Repository<Servant> {
        Repository::new(self.database.clone())
    }

    pub fn profile_repository(&self) -> Repository<Profile> {
        Repository::new(self.database.clone())
    }
}
