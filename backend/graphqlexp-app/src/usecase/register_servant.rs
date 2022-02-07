use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;

use graphqlexp_adapter::{
    modules::RepositoriesModule,
    repositories::servant::{Class, NewServant, Rarity, Servant, ServantRepository},
};

pub struct ServantRegistration {
    pub name: String,
    pub class_name: String,
    pub rarity: i32,
}

pub struct RegisterServant {
    repositories: Arc<RepositoriesModule>,
}

impl RegisterServant {
    pub fn new(repositories: &Arc<RepositoriesModule>) -> Self {
        Self {
            repositories: repositories.clone(),
        }
    }

    pub async fn execute(&self, registration: ServantRegistration) -> Result<Servant> {
        let repository = self.repositories.servant_repository();
        let new_servant = NewServant {
            name: registration.name,
            class: Class::from_str(&registration.class_name)?,
            rarity: Rarity::create(registration.rarity)?,
        };
        let result = repository.register(new_servant).await?;
        Ok(result)
    }
}
