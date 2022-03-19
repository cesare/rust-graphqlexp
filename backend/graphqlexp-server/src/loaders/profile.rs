use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use dataloader::{
    BatchFn,
    non_cached::Loader,
};

use graphqlexp_app::{
    models::{
        profile::Profile,
        servant::ServantId,
    },
    modules::RepositoriesModule,
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

    fn complete_profile_map(&self, ids: &[ServantId], map: ProfileMap) -> ProfileMap {
        let mut target: ProfileMap = HashMap::new();
        for id in ids {
            match map.get(id) {
                Some(profiles) => {
                    target.insert(id.to_owned(), profiles.to_vec());
                },
                _ => {
                    target.insert(id.to_owned(), vec![]);
                },
            }
        }
        target
    }
}

#[async_trait]
impl BatchFn<ServantId, Vec<Profile>> for ServantProfilesLoadFn {
    async fn load(&mut self, keys: &[ServantId]) -> ProfileMap {
        let profiles = self.load_profiles(keys).await.unwrap();
        let profile_map = self.create_servant_profiles(profiles);
        self.complete_profile_map(keys, profile_map)
    }
}

#[allow(dead_code)]
pub type ServantProfilesLoader = Loader<ServantId, Vec<Profile>, ServantProfilesLoadFn>;

pub struct LoaderFactory {
    repositories: RepositoriesModule,
}

impl LoaderFactory {
    pub fn new(repositories: &RepositoriesModule) -> Self {
        Self {
            repositories: repositories.clone(),
        }
    }

    pub fn servant_profiles_loader(&self) -> ServantProfilesLoader {
        let load_fn = ServantProfilesLoadFn {
            profile_repository: self.repositories.profile_repository(),
        };
        Loader::new(load_fn)
    }
}
