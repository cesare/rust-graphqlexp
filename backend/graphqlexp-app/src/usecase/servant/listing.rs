use std::sync::Arc;

use graphqlexp_adapter::{
    modules::RepositoriesModule,
    repositories::servant::ServantRepository,
};

use crate::models::servant::Servant;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct ListingServants {
    repositories: Arc<RepositoriesModule>,
}

impl ListingServants {
    pub fn new(repositories: &Arc<RepositoriesModule>) -> Self {
        Self {
            repositories: repositories.clone(),
        }
    }

    pub async fn execute(&self) -> Result<Vec<Servant>> {
        let repository = self.repositories.servant_repository();
        let servants = repository.list().await?;
        Ok(servants)
    }
}
