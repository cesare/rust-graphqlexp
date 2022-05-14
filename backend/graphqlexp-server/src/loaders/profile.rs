use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{
    BatchFn,
    non_cached::Loader,
};

use graphqlexp_app::{
    models::{
        profile::Profile,
        servant::{Servant, ServantId},
    },
    modules::RepositoriesModule,
    repositories::{
        profile::ProfileRepository,
        Repository,
    },
};

use crate::loaders::map::OneToManyMap;

type ProfileMap = HashMap<ServantId, Vec<Profile>>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
        let mut map = OneToManyMap::<Servant, Profile>::new(keys);
        map.insert_all(&profiles);
        map.finish()
    }
}

pub type ServantProfilesLoader = Loader<ServantId, Vec<Profile>, ServantProfilesLoadFn>;

pub(super) fn servant_profiles_loader(repositories: &RepositoriesModule) -> ServantProfilesLoader {
    let load_fn = ServantProfilesLoadFn {
        profile_repository: repositories.profile_repository(),
    };
    Loader::new(load_fn)
}
