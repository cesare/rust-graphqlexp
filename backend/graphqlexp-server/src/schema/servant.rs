use juniper::{
    GraphQLInputObject,
    GraphQLObject,
};

use graphqlexp_app::usecase::ServantRegistration;

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

impl From<ServantInput> for ServantRegistration {
    fn from(input: ServantInput) -> Self {
        Self {
            name: input.name,
            class_name: input.class_name,
            rarity: input.rarity,
        }
    }
}
