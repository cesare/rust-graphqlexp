use juniper::{
    EmptySubscription,
    FieldError,
    FieldResult,
    RootNode,
    graphql_value
};

use graphqlexp_app::{
    modules::UsecasesModule,
};

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
        let usecase = context.usecases.show_servant_usecase();
        let result = usecase.find(id).await?;

        match result {
            Some(servant) => {
                Ok(Servant {
                    id: servant.id.value,
                    name: servant.name,
                    class_name: servant.class.to_string(),
                    rarity: servant.rarity.value(),
                })
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
        let usecase = context.usecases.list_servants_usecase();
        let servants = usecase.execute().await?;

        let results = servants.iter().map(|servant|
            Servant {
                id: servant.id.value,
                name: servant.name.to_owned(),
                class_name: servant.class.to_string(),
                rarity: servant.rarity.value(),
            }
        ).collect();
        Ok(results)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_servant(context: &Context, input: ServantInput) -> FieldResult<Servant> {
        let usecase = context.usecases.register_servant_usecase();
        let servant = usecase.execute(input.into()).await?;

        Ok(Servant {
            id: servant.id.value,
            name: servant.name.to_owned(),
            class_name: servant.class.to_string(),
            rarity: servant.rarity.value(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
