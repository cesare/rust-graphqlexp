use std::path::PathBuf;

use actix_web::{App, HttpServer, web};
use anyhow::Result;
use structopt::StructOpt;

use graphqlexp_app::{
    modules::{UsecasesModule},
};
use graphqlexp_server_rusty_gql::{
    config::{GraphqlexpConfig},
    logger::{initialize_logger, default_logger},
    routes::RoutesFactory,
};

#[derive(StructOpt)]
#[structopt(name = "graphqlexp-rusty-gql")]
struct Args {
    #[structopt(short, long, parse(from_os_str))]
    config_file: PathBuf,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let config = GraphqlexpConfig::load(&args.config_file).await?;
    let usecases = UsecasesModule::create(&config.database).await?;

    initialize_logger()?;

    let bind_address = config.server.bind_address();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(default_logger())
            .app_data(web::Data::new(usecases.clone()))
            .configure(RoutesFactory::create(&config))
    });
    server.bind(bind_address)?.run().await?;

    Ok(())
}
