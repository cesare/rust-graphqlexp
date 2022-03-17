use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use dataloader::BatchFn;

use graphqlexp_app::{
    models::{
        profile::Profile,
        servant::ServantId,
    },
    repositories::{
        profile::ProfileRepository,
        Repository,
    },
};

type ProfileMap = HashMap<ServantId, Vec<Profile>>;

pub struct ServantProfilesLoadFn {
    profile_repository: Repository<Profile>,
}

impl ServantProfilesLoadFn {
    pub async fn load_profiles(&self, ids: &[ServantId]) -> Result<Vec<Profile>> {
        self.profile_repository.list_for_servants(ids).await
    }

    fn create_servant_profiles(&self, profiles: Vec<Profile>) -> ProfileMap {
        let mut map: ProfileMap = HashMap::new();
        for profile in profiles {
            let servant_id = profile.servant_id.clone();

            match map.get_mut(&servant_id) {
                Some(ps) => ps.push(profile),
                None => {
                    let ps: Vec<Profile> = vec![profile];
                    map.insert(servant_id, ps);
                }
            }
        }
        map
    }
}

#[async_trait]
impl BatchFn<ServantId, Vec<Profile>> for ServantProfilesLoadFn {
    async fn load(&mut self, keys: &[ServantId]) -> ProfileMap {
        let profiles = self.load_profiles(keys).await.unwrap();
        self.create_servant_profiles(profiles)
    }
}
