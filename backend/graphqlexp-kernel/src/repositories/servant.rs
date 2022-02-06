use anyhow::Result;
use async_trait::async_trait;

use crate::models::{
    servant::{Class, Rarity, Servant, ServantId}
};

pub struct NewServant {
    pub name: String,
    pub class: Class,
    pub rarity: Rarity,
}

#[async_trait]
pub trait ServantRepository {
    async fn find(&self, id: ServantId) -> Result<Option<Servant>>;
    async fn list(&self) -> Result<Vec<Servant>>;
    async fn register(&self, servant: NewServant) -> Result<Servant>;
}
