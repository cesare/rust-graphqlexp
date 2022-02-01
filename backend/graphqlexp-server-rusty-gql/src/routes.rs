use actix_web::{
    Error, HttpResponse,
    web::{
        Data, Json, ServiceConfig,
        post, resource,
    }
};
use anyhow::Result;

use graphqlexp_app::modules::UsecasesModule;
use crate::config::GraphqlexpConfig;

pub struct RoutesFactory;

impl RoutesFactory {
    pub fn create(_config: &GraphqlexpConfig) -> impl FnOnce(&mut ServiceConfig) {
        |cfg: &mut ServiceConfig| {
            cfg.service(
                resource("/graphql").route(post().to(graphql))
            );
        }
    }
}

async fn graphql(usecases: Data<UsecasesModule>) -> Result<HttpResponse, Error> {
    todo!()
}
