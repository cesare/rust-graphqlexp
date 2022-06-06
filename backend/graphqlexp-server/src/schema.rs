use graphqlexp_app::{
    models::{
        profile::Profile as ProfileModel,
    },
    modules::{RepositoriesModule, UsecasesModule},
    repositories::{
        Repository,
    },
    usecase::{
        ProfileRegistration,
        RegisterServant,
    }
};

use crate::loaders::Loaders;

pub mod root;
mod profile;
mod servant;

pub use root::Schema;

pub struct Context {
    repositories: RepositoriesModule,
    pub loaders: Loaders,
}

impl Context {
    pub fn new(repositories: &RepositoriesModule) -> Self {
        Self {
            repositories: repositories.clone(),
            loaders: Loaders::new(repositories),
        }
    }

    pub fn profile_repository(&self) -> Repository<ProfileModel> {
        self.repositories.profile_repository()
    }

    pub fn usecases(&self) -> UsecasesModule {
        UsecasesModule::new(self.repositories.clone())
    }

    pub fn register_servant_usecase(&self) -> RegisterServant {
        let usecases = UsecasesModule::new(self.repositories.clone());
        usecases.register_servant_usecase()
    }

    pub fn profile_registration_usecase(&self) -> ProfileRegistration {
        let usecases = UsecasesModule::new(self.repositories.clone());
        usecases.profile_registration_usecase()
    }
}

impl juniper::Context for Context {}
