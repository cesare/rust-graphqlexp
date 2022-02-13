use std::sync::Arc;

use anyhow::Result;

use graphqlexp_adapter::{
    modules::RepositoriesModule,
    repositories::profile::{NewProfile, Profile, ProfileNumber, ProfileRepository},
    repositories::servant::ServantId,
};

pub struct ProfileAttributes {
    pub servant_id: i32,
    pub number: i32,
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
            number: ProfileNumber::create(attrs.number)?,
            text: attrs.text,
        };
        let profile = repository.register(new_profile).await?;
        Ok(profile)
    }
}
