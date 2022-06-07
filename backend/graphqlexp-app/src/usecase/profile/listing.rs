use std::sync::Arc;

use graphqlexp_adapter::{
    modules::RepositoriesModule,
    repositories::profile::ProfileRepository,
};

use crate::models::{
    profile::Profile,
    servant::ServantId,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct ProfilesForServants {
    repositories: Arc<RepositoriesModule>,
}

impl ProfilesForServants {
    pub fn new(repositories: &Arc<RepositoriesModule>) -> Self {
        Self {
            repositories: repositories.clone(),
        }
    }

    pub async fn list(&self, ids: &[ServantId]) -> Result<Vec<Profile>> {
        let repository = self.repositories.profile_repository();
        let profiles = repository.list_for_servants(ids).await?;
        Ok(profiles)
    }
}
