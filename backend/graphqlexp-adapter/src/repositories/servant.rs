use anyhow::Result;
use async_trait::async_trait;

use graphqlexp_kernel::{
    models::servant::{Servant, ServantId},
    repositories::servant::ServantRepository,
};
use super::Repository;

#[async_trait]
impl ServantRepository for Repository<Servant> {
    #[allow(unused_variables)]
    async fn find(&self, id: ServantId) -> Result<Option<Servant>> {
        todo!()
    }
}
