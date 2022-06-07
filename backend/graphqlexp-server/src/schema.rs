use graphqlexp_app::{
    modules::UsecasesModule,
};

use crate::loaders::Loaders;

pub mod root;
mod profile;
mod servant;

pub use root::Schema;

pub struct Context {
    usecases: UsecasesModule,
    pub loaders: Loaders,
}

impl Context {
    pub fn new(usecases: &UsecasesModule) -> Self {
        Self {
            usecases: usecases.clone(),
            loaders: Loaders::new(&usecases),
        }
    }

    pub fn usecases(&self) -> UsecasesModule {
        self.usecases.clone()
    }
}

impl juniper::Context for Context {}
