use juniper::{EmptySubscription, EmptyMutation, FieldResult, GraphQLObject, RootNode};

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
    fn servant(_context: &Context, _id: i32) -> FieldResult<Servant> {
        Ok(Servant {
            id: 1,
            name: "Meltryllis".to_owned(),
            class_name: "alterego".to_owned(),
            rarity: 5,
        })
    }
}


pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
