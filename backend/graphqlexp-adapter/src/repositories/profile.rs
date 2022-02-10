use anyhow::Result;
use async_trait::async_trait;

pub use graphqlexp_kernel::{
    models::{
        profile::{Profile, ProfileId, ProfileNumber},
        servant::ServantId
    },
    repositories::profile::{NewProfile, ProfileRepository},
};
use super::Repository;

#[async_trait]
impl ProfileRepository for Repository<Profile> {
    async fn find(&self, _id: &ProfileId) -> Result<Option<Profile>> {
        todo!()
    }

    async fn list_for_servant(&self, _servant_id: &ServantId) -> Result<Vec<Profile>> {
        todo!()
    }

    async fn register(&self, _profile: NewProfile) -> Result<Profile> {
        todo!()
    }
}
