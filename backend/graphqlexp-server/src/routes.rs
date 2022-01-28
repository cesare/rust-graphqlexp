use actix_web::{
    Error, HttpResponse,
    web::{
        Data, Json, ServiceConfig,
        block, post, resource,
    }
};
use anyhow::Result;
use juniper::http::GraphQLRequest;

use graphqlexp_app::modules::UsecasesModule;
use crate::schema::{Context, Schema};

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/graphql").route(post().to(graphql))
    );
}

async fn graphql(usecases: Data<UsecasesModule>, schema: Data<Schema>, data: Json<GraphQLRequest>) -> Result<HttpResponse, Error> {
    let context = Context {
        usecases: usecases.get_ref().to_owned(),
    };

    let servant = block(move || {
        let res = data.execute_sync(&schema, &context);
        serde_json::to_string(&res)
    }).await??;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(servant))
}
