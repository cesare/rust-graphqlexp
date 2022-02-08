use juniper::{
    EmptySubscription,
    FieldError,
    FieldResult,
    GraphQLInputObject,
    GraphQLObject,
    RootNode,
    graphql_value
};

use graphqlexp_app::modules::UsecasesModule;

#[derive(GraphQLObject)]
struct Servant {
    id: i32,
    name: String,
    class_name: String,
    rarity: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Servant Input")]
pub struct ServantInput {
    pub name: String,
    pub class_name: String,
    pub rarity: i32,
}

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
    async fn create_servant(_context: &Context, _input: ServantInput) -> FieldResult<Servant> {
        todo!()
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
