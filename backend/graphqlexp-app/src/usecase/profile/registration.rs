use std::sync::Arc;

use anyhow::Result;

use graphqlexp_adapter::{
    models::{
        profile::{Profile, ProfilePosition},
        servant::{ServantId},
    },
    modules::RepositoriesModule,
    repositories::profile::{NewProfile, ProfileRepository},
};

pub struct ProfileAttributes {
    pub servant_id: i32,
    pub position: i32,
    pub text: String,
}

pub struct ProfileRegistration {
    repositories: Arc<RepositoriesModule>,
}

impl ProfileRegistration {
    pub fn new(repositories: &Arc<RepositoriesModule>) -> Self {
        Self {
            repositories: repositories.clone(),
        }
    }

    pub async fn execute(&self, attrs: ProfileAttributes) -> Result<Profile> {
        let repository = self.repositories.profile_repository();
        let new_profile = NewProfile {
            servant_id: ServantId::new(attrs.servant_id),
            position: ProfilePosition::create(attrs.position)?,
            text: attrs.text,
        };
        let profile = repository.register(new_profile).await?;
        Ok(profile)
    }
}
