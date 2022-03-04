use juniper::GraphQLInputObject;

use graphqlexp_app::{
    models::servant::Servant as ServantModel,
    repositories::profile::ProfileRepository,
    usecase::ServantRegistration,
};

use crate::schema::{
    Context,
    profile::Profile,
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

    async fn profiles(&self, context: &Context) -> Vec<Profile> {
        let repository = context.usecases.repositories.profile_repository();
        repository.list_for_servant(&self.model.id)
            .await
            .unwrap()
            .into_iter()
            .map(|result| result.into())
            .collect()
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
