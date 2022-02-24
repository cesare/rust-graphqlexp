use juniper::{
    GraphQLInputObject,
    GraphQLObject,
};

use graphqlexp_app::models::servant::{
    Servant as ServantModel,
};
use graphqlexp_app::usecase::{
    ServantRegistration,
};

#[derive(GraphQLObject)]
pub(super) struct Servant {
    id: i32,
    name: String,
    class_name: String,
    rarity: i32,
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
pub(super) struct ServantInput {
    name: String,
    class_name: String,
    rarity: i32,
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
