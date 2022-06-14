use juniper::GraphQLInputObject;

use graphqlexp_app::{
    models::profile::Profile as ProfileModel,
    usecase::ProfileAttributes,
};

pub(super) struct Profile {
    model: ProfileModel,
}

#[juniper::graphql_object]
impl Profile {
    fn id(&self) -> &str {
        &self.model.id.value
    }

    fn position(&self) -> i32 {
        self.model.position.value()
    }

    fn text(&self) -> &str {
        self.model.text.as_ref()
    }
}

impl From<ProfileModel> for Profile {
    fn from(model: ProfileModel) -> Self {
        Self {
            model,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Profile Input")]
pub(super) struct ProfileInput {
    servant_id: String,
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
