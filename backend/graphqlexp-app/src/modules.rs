use std::sync::Arc;

use anyhow::Result;

pub use graphqlexp_adapter::modules::RepositoriesModuleConfig;
use graphqlexp_adapter::modules::{RepositoriesModule};
use crate::usecase::ShowServant;

pub struct UsecasesModule {
    repositories: Arc<RepositoriesModule>,
}

impl UsecasesModule {
    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self> {
        let repositories = RepositoriesModule::create(config).await?;
        Ok(Self { repositories: Arc::new(repositories) })
    }

    pub fn show_servant_usecase(&self) -> ShowServant {
        ShowServant::new(&self.repositories)
    }
}