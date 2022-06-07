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
    modules::UsecasesModule,
};

use crate::loaders::map::OneToManyMap;

type ProfileMap = HashMap<ServantId, Vec<Profile>>;

pub struct ServantProfilesLoadFn {
    usecases: UsecasesModule,
}

impl ServantProfilesLoadFn {
    pub async fn load_profiles(&self, ids: &[ServantId]) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
        let usecase = self.usecases.listing_profiles_for_servants_usecase();
        usecase.list(ids).await
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

pub(super) fn servant_profiles_loader(usecases: &UsecasesModule) -> ServantProfilesLoader {
    let load_fn = ServantProfilesLoadFn {
        usecases: usecases.clone(),
    };
    Loader::new(load_fn)
}
