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

struct HasManyMap<K, T> {
    map: HashMap<K, Vec<T>>,
}

impl<K, T> HasManyMap<K, T> where K: Clone + Eq + std::hash::Hash {
    fn new(keys: &[K]) -> Self {
        let mut map = HashMap::with_capacity(keys.len());
        for key in keys {
            map.insert(key.clone(), vec![]);
        }

        Self { map }
    }

    fn insert(&mut self, key: &K, value: T) {
        let values = self.map.get_mut(key).unwrap();
        values.push(value);
    }

    fn finish(self) -> HashMap<K, Vec<T>> {
        self.map
    }
}

pub struct ServantProfilesLoadFn {
    profile_repository: Repository<Profile>,
}

impl ServantProfilesLoadFn {
    pub async fn load_profiles(&self, ids: &[ServantId]) -> Result<Vec<Profile>> {
        self.profile_repository.list_for_servants(ids).await
    }
}

#[async_trait]
impl BatchFn<ServantId, Vec<Profile>> for ServantProfilesLoadFn {
    async fn load(&mut self, keys: &[ServantId]) -> ProfileMap {
        let profiles = self.load_profiles(keys).await.unwrap();
        let mut map = HasManyMap::<ServantId, Profile>::new(keys);
        for profile in profiles {
            map.insert(&profile.servant_id, profile.clone());
        }
        map.finish()
    }
}

#[allow(dead_code)]
pub type ServantProfilesLoader = Loader<ServantId, Vec<Profile>, ServantProfilesLoadFn>;

pub(super) fn servant_profiles_loader(repositories: &RepositoriesModule) -> ServantProfilesLoader {
    let load_fn = ServantProfilesLoadFn {
        profile_repository: repositories.profile_repository(),
    };
    Loader::new(load_fn)
}
