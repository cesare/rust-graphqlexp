use actix_web::{
    Error, HttpResponse,
    web::{
        Data, Json, ServiceConfig,
        post, resource,
    }
};
use anyhow::Result;

use graphqlexp_app::modules::UsecasesModule;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/graphql").route(post().to(graphql))
    );
}

async fn graphql(usecases: Data<UsecasesModule>) -> Result<HttpResponse, Error> {
    todo!()
}
