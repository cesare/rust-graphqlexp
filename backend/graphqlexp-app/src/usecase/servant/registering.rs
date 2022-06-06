use std::sync::Arc;

use graphqlexp_adapter::{
    models::servant::{Class, Rarity, Servant},
    modules::RepositoriesModule,
    repositories::servant::{NewServant, ServantRepository},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
            class: Class::from(registration.class_name.as_str()),
            rarity: Rarity::new(registration.rarity),
        };
        let result = repository.register(new_servant).await?;
        Ok(result)
    }
}
