use std::convert::From;

use juniper::{
    EmptySubscription,
    FieldError,
    FieldResult,
    RootNode,
    graphql_value
};

use graphqlexp_app::{
    modules::UsecasesModule,
    repositories::servant::ServantRepository,
};

mod profile;
use profile::{Profile, ProfileInput};
mod servant;
use servant::{Servant, ServantInput};

pub struct Context {
    pub usecases: UsecasesModule,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn servant(context: &Context, id: i32) -> FieldResult<Servant> {
        let repository = context.usecases.repositories.servant_repository();
        let result = repository.find(id.into()).await?;

        match result {
            Some(servant) => {
                Ok(servant.into())
            }
            _ => {
                Err(FieldError::new(
                    "Servant Not Found",
                    graphql_value!({ "not_found": "servant not found" }),
                ))
            }
        }
    }

    async fn list_servants(context: &Context) -> FieldResult<Vec<Servant>> {
        let repository = context.usecases.repositories.servant_repository();
        let servants = repository.list().await?;

        let results = servants.into_iter()
            .map(|servant| servant.into())
            .collect();
        Ok(results)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_servant(context: &Context, input: ServantInput) -> FieldResult<Servant> {
        let usecase = context.usecases.register_servant_usecase();
        let servant = usecase.execute(input.into()).await?;
        Ok(servant.into())
    }

    async fn register_profile(context: &Context, input: ProfileInput) -> FieldResult<Profile> {
        let usecase = context.usecases.profile_registration_usecase();
        let profile = usecase.execute(input.into()).await?;
        Ok(profile.into())
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
