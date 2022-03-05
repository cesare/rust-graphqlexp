use anyhow::Result;
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
    async fn find(&self, id: &ProfileId) -> Result<Option<Profile>>;
    async fn list_for_servant(&self, servant_id: &ServantId) -> Result<Vec<Profile>>;
    async fn list_for_servants(&self, ids: &[ServantId]) -> Result<Vec<Profile>>;
    async fn register(&self, profile: NewProfile) -> Result<Profile>;
}
