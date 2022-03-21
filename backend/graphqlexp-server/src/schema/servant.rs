use juniper::{FieldResult, GraphQLInputObject};

use graphqlexp_app::{
    models::servant::Servant as ServantModel,
    usecase::ServantRegistration,
};

use crate::{
    schema::{
        Context,
        profile::Profile,
    },
};

pub(super) struct Servant {
    model: ServantModel,
}

#[juniper::graphql_object(Context = Context)]
impl Servant {
    fn id(&self) -> i32 {
        self.model.id.value
    }

    fn name(&self) -> &str {
        &self.model.name
    }

    fn class_name(&self) -> String {
        self.model.class.to_string()
    }

    fn rarity(&self) -> i32 {
        self.model.rarity.value()
    }

    async fn profiles(&self, context: &Context) -> FieldResult<Vec<Profile>> {
        let keys = vec![self.model.id.clone()];

        let mut profile_map = context.loaders.servant_profiles_loader
            .load_many(keys)
            .await;
        let profiles = profile_map.remove(&self.model.id).unwrap()
            .into_iter()
            .map(|profile| profile.into())
            .collect();
        Ok(profiles)
    }
}

impl From<ServantModel> for Servant {
    fn from(model: ServantModel) -> Self {
        Self {
            model,
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
