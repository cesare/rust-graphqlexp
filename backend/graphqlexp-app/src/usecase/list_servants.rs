use std::sync::Arc;

use anyhow::Result;

use graphqlexp_adapter::{
    models::servant::Servant,
    modules::RepositoriesModule,
    repositories::servant::ServantRepository,
};

pub struct ListServants {
    repositories: Arc<RepositoriesModule>,
}

impl ListServants {
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
