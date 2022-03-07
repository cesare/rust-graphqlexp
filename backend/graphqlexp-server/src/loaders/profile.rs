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
}

#[async_trait]
impl BatchFn<ServantId, Vec<Profile>> for ServantProfilesLoader {
    async fn load(&mut self, _keys: &[ServantId]) -> HashMap<ServantId, Vec<Profile>> {
        todo!()
    }
}
