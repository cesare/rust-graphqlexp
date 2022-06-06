use graphqlexp_app::{
    models::{
        profile::Profile as ProfileModel,
    },
    modules::{RepositoriesModule, UsecasesModule},
    repositories::{
        Repository,
    },
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
}

impl juniper::Context for Context {}
