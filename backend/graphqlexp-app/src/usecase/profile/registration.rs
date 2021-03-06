use std::{sync::Arc, str::FromStr};

use graphqlexp_adapter::{
    modules::RepositoriesModule,
    repositories::profile::{NewProfile, ProfileRepository}, models::profile::ProfileText,
};

use crate::models::{
    profile::{Profile, ProfilePosition},
    servant::ServantId,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct ProfileAttributes {
    pub servant_id: String,
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
            position: ProfilePosition::new(attrs.position),
            text: ProfileText::from_str(&attrs.text)?,
        };
        let profile = repository.register(new_profile).await?;
        Ok(profile)
    }
}
