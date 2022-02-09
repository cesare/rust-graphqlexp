use juniper::{
    GraphQLInputObject,
    GraphQLObject,
};

#[derive(GraphQLObject)]
pub(crate) struct Servant {
    pub id: i32,
    pub name: String,
    pub class_name: String,
    pub rarity: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Servant Input")]
pub(crate) struct ServantInput {
    pub name: String,
    pub class_name: String,
    pub rarity: i32,
}
