use juniper::{EmptySubscription, EmptyMutation, FieldError, FieldResult, GraphQLObject, RootNode, graphql_value};

use graphqlexp_app::modules::UsecasesModule;

#[derive(GraphQLObject)]
struct Servant {
    id: i32,
    name: String,
    class_name: String,
    rarity: i32,
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
                    rarity: servant.rarity,
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
                rarity: servant.rarity,
            }
        ).collect();
        Ok(results)
    }
}


pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
