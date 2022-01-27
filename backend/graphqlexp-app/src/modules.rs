use anyhow::Result;

use graphqlexp_adapter::modules::{RepositoriesModule, RepositoriesModuleConfig};
use crate::usecase::ShowServant;

pub struct UsercasesModule {
    repositories: RepositoriesModule,
}

impl UsercasesModule {
    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self> {
        let repositories = RepositoriesModule::create(config).await?;

        Ok(Self { repositories: repositories })
    }
}
