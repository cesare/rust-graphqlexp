use std::sync::Arc;

use anyhow::Result;

pub use graphqlexp_adapter::modules::RepositoriesModuleConfig;
use graphqlexp_adapter::modules::{RepositoriesModule};
use crate::usecase::{ProfileRegistration, RegisterServant};

#[derive(Clone)]
pub struct UsecasesModule {
    pub repositories: Arc<RepositoriesModule>,
}

impl UsecasesModule {
    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self> {
        let repositories = RepositoriesModule::create(config).await?;
        Ok(Self { repositories: Arc::new(repositories) })
    }

    pub fn register_servant_usecase(&self) -> RegisterServant {
        RegisterServant::new(&self.repositories)
    }

    pub fn profile_registration_usecase(&self) -> ProfileRegistration {
        ProfileRegistration::new(&self.repositories)
    }
}
