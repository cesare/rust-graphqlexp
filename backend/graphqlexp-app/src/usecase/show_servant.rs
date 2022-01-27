use std::sync::Arc;

use anyhow::Result;

use graphqlexp_adapter::{
    modules::RepositoriesModule,
    repositories::servant::{Servant, ServantId, ServantRepository},
};

pub struct ShowServant {
    repositories: Arc<RepositoriesModule>,
}

impl ShowServant {
    pub fn new(repositories: &Arc<RepositoriesModule>) -> Self {
        Self {
            repositories: repositories.clone(),
        }
    }

    pub async fn find(&self, id: u32) -> Result<Option<Servant>> {
        let repository = self.repositories.servant_repository();
        let result = repository.find(ServantId::new(id)).await?;
        match result {
            Some(servant) => Ok(Some(servant)),
            _ => Ok(None),
        }
    }
}
