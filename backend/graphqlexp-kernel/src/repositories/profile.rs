use async_trait::async_trait;

use crate::models::{
    profile::{Profile, ProfileId, ProfilePosition},
    servant::ServantId,
};

pub struct NewProfile {
    pub servant_id: ServantId,
    pub position: ProfilePosition,
    pub text: String,
}

#[async_trait]
pub trait ProfileRepository {
    type Error: std::error::Error;

    async fn find(&self, id: &ProfileId) -> Result<Option<Profile>, Self::Error>;
    async fn list_for_servant(&self, servant_id: &ServantId) -> Result<Vec<Profile>, Self::Error>;
    async fn list_for_servants(&self, ids: &[ServantId]) -> Result<Vec<Profile>, Self::Error>;
    async fn register(&self, profile: NewProfile) -> Result<Profile, Self::Error>;
}
