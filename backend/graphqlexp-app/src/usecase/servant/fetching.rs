use std::sync::Arc;

use graphqlexp_adapter::{
    models::servant::{Servant, ServantId},
    modules::RepositoriesModule,
    repositories::servant::ServantRepository,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct FetchingServant {
    repositories: Arc<RepositoriesModule>,
}

impl FetchingServant {
    pub fn new(repositories: &Arc<RepositoriesModule>) -> Self {
        Self {
            repositories: repositories.clone(),
        }
    }

    pub async fn execute(&self, id: ServantId) -> Result<Option<Servant>> {
        let repository = self.repositories.servant_repository();
        let servant = repository.find(id).await?;
        Ok(servant)
    }
}
