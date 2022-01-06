use async_trait::async_trait;

use crate::models::{
    servant::{Servant, ServantId}
};

#[async_trait]
pub trait ServantRepository {
    async fn find(&self, id: ServantId) -> anyhow::Result<Option<Servant>>;
}
