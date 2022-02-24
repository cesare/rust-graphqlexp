use juniper::{
    GraphQLInputObject,
    GraphQLObject,
};

use graphqlexp_app::models::profile::{
    Profile as ProfileModel,
};
use graphqlexp_app::usecase::{
    ProfileAttributes,
};

#[derive(GraphQLObject)]
pub(super) struct Profile {
    id: String,
    servant_id: i32,
    position: i32,
    text: String,
}

impl From<ProfileModel> for Profile {
    fn from(model: ProfileModel) -> Self {
        Self {
            id: model.id.value,
            servant_id: model.servant_id.value,
            position: model.position.value(),
            text: model.text,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Profile Input")]
pub(super) struct ProfileInput {
    servant_id: i32,
    position: i32,
    text: String,
}

impl From<ProfileInput> for ProfileAttributes {
    fn from(input: ProfileInput) -> Self {
        Self {
            servant_id: input.servant_id,
            position: input.position,
            text: input.text,
        }
    }
}
