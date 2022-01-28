use actix_web::{
    Error, HttpResponse,
    web::{
        Data, Json, ServiceConfig,
        block, post, resource,
    }
};
use anyhow::Result;
use juniper::http::GraphQLRequest;

use crate::schema::Schema;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/graphql").route(post().to(graphql))
    );
}

async fn graphql(schema: Data<Schema>, data: Json<GraphQLRequest>) -> Result<HttpResponse, Error> {
    let servant = block(move || {
        let res = data.execute_sync(&schema, &());
        serde_json::to_string(&res)
    }).await??;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(servant))
}
