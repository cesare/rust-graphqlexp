use juniper::{
    EmptySubscription,
    FieldError,
    FieldResult,
    RootNode,
    graphql_value
};

use graphqlexp_app::repositories::servant::ServantRepository;

use super::{
    Context,
    profile::{Profile, ProfileInput},
    servant::{Servant, ServantInput},
};

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn servant(context: &Context, id: String) -> FieldResult<Servant> {
        let usecase = context.usecases().fetching_servant_usecase();
        let result = usecase.execute(id.into()).await?;

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
        let repository = context.servant_repository();
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
        let usecase = context.register_servant_usecase();
        let servant = usecase.execute(input.into()).await?;
        Ok(servant.into())
    }

    async fn register_profile(context: &Context, input: ProfileInput) -> FieldResult<Profile> {
        let usecase = context.profile_registration_usecase();
        let profile = usecase.execute(input.into()).await?;
        Ok(profile.into())
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
