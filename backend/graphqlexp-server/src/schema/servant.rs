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
    id: i32,
    name: String,
    class_name: String,
    rarity: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Servant {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn class_name(&self) -> &str {
        &self.class_name
    }

    fn rarity(&self) -> i32 {
        self.rarity
    }

    async fn profiles(&self, context: &Context) -> Vec<Profile> {
        let repository = context.usecases.repositories.profile_repository();
        repository.list_for_servant(&self.id.into())
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
