use std::sync::Arc;

pub use graphqlexp_adapter::modules::RepositoriesModuleConfig;
pub use graphqlexp_adapter::modules::RepositoriesModule;
use crate::usecase::{ListingServants, ProfileRegistration, RegisterServant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
pub struct UsecasesModule {
    pub repositories: Arc<RepositoriesModule>,
}

impl UsecasesModule {
    pub fn new(repositories: RepositoriesModule) -> Self {
        Self {
            repositories: Arc::new(repositories)
        }
    }

    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self> {
        let repositories = RepositoriesModule::create(config).await?;
        Ok(Self::new(repositories))
    }

    pub fn listing_servant_usecase(&self) -> ListingServants {
        ListingServants::new(&self.repositories)
    }

    pub fn register_servant_usecase(&self) -> RegisterServant {
        RegisterServant::new(&self.repositories)
    }

    pub fn profile_registration_usecase(&self) -> ProfileRegistration {
        ProfileRegistration::new(&self.repositories)
    }
}
