use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use juniper::http::GraphQLRequest;

use crate::schema::Schema;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to(graphql)));
}

async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> Result<HttpResponse, Error> {
    let servant = web::block(move || {
        let res = data.execute_sync(&st, &());
        serde_json::to_string(&res)
    }).await??;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(servant))
}
