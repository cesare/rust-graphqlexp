use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use dataloader::BatchFn;

use graphqlexp_app::models::{
    profile::Profile,
    servant::ServantId,
};

pub struct ServantProfilesLoader {}

impl ServantProfilesLoader {
    pub async fn load_profiles(&self, _ids: &[ServantId]) -> Result<Vec<Profile>> {
        todo!()
    }

    fn create_servant_profiles(&self, profiles: Vec<Profile>) -> HashMap<ServantId, Vec<Profile>> {
        let mut map: HashMap<ServantId, Vec<Profile>> = HashMap::new();
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
impl BatchFn<ServantId, Vec<Profile>> for ServantProfilesLoader {
    async fn load(&mut self, keys: &[ServantId]) -> HashMap<ServantId, Vec<Profile>> {
        let profiles = self.load_profiles(keys).await.unwrap();
        self.create_servant_profiles(profiles)
    }
}
