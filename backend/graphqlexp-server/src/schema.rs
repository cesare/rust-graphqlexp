use juniper::{EmptySubscription, EmptyMutation, FieldResult, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
struct Servant {
    id: i32,
    name: String,
    class_name: String,
    rarity: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn servant(_id: i32) -> FieldResult<Servant> {
        Ok(Servant {
            id: 1,
            name: "Meltryllis".to_owned(),
            class_name: "alterego".to_owned(),
            rarity: 5,
        })
    }
}


pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
