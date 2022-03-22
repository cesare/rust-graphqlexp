use actix_web::{
    Error, HttpResponse,
    web::{
        Data, Json, ServiceConfig,
        post, resource,
    }
};
use anyhow::Result;
use juniper::http::GraphQLRequest;

use graphqlexp_app::modules::RepositoriesModule;
use crate::schema::{Context, Schema};

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/graphql").route(post().to(graphql))
    );
}

async fn graphql(repositories: Data<RepositoriesModule>, schema: Data<Schema>, data: Json<GraphQLRequest>) -> Result<HttpResponse, Error> {
    let context = Context::new(repositories.as_ref());

    let res = data.execute(&schema, &context).await;
    let servant = serde_json::to_string(&res)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(servant))
}
