use async_trait::async_trait;

use crate::models::{
    id::Id,
    servant::Servant
};

#[async_trait]
pub trait ServantRepository {
    async fn find(&self, id: Id<Servant, u32>) -> anyhow::Result<Option<Servant>>;
}
