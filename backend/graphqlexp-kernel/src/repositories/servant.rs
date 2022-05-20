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
    type Error: std::error::Error;

    async fn find(&self, id: ServantId) -> Result<Option<Servant>, Self::Error>;
    async fn list(&self) -> Result<Vec<Servant>, Self::Error>;
    async fn register(&self, servant: NewServant) -> Result<Servant, Self::Error>;
}
