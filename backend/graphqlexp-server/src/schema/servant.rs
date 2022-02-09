use juniper::{
    GraphQLInputObject,
    GraphQLObject,
};

use graphqlexp_app::usecase::{
    ServantRegistration,
    Servant as ServantModel,
};

#[derive(GraphQLObject)]
pub(crate) struct Servant {
    pub id: i32,
    pub name: String,
    pub class_name: String,
    pub rarity: i32,
}

impl From<ServantModel> for Servant {
    fn from(model: ServantModel) -> Self {
        Self {
            id: model.id.value,
            name: model.name.to_owned(),
            class_name: model.class.to_string(),
            rarity: model.rarity.value(),
        }
    }
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
