use std::sync::Arc;

use anyhow::Result;

use graphqlexp_adapter::{
    modules::RepositoriesModuleExt,
    repositories::servant::{Servant, ServantId, ServantRepository},
};

pub struct ShowServant<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> ShowServant<R> {
    pub fn new(repositories: Arc<R>) -> Self {
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
