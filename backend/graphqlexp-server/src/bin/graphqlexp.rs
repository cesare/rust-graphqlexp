use std::path::PathBuf;

use actix_web::{App, HttpServer, web};
use structopt::StructOpt;

use graphqlexp_app::{
    modules::{RepositoriesModule, UsecasesModule},
};
use graphqlexp_server::{
    config::{GraphqlexpConfig},
    logger::{initialize_logger, default_logger},
    routes::configure_routes,
    schema::root::create_schema,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt)]
#[structopt(name = "graphqlexp")]
struct Args {
    #[structopt(short, long, parse(from_os_str))]
    config_file: PathBuf,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let config = GraphqlexpConfig::load(&args.config_file).await?;
    let repositories = RepositoriesModule::create(&config.database).await?;
    let usecases = UsecasesModule::create(&config.database).await?;

    initialize_logger()?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(default_logger())
            .app_data(web::Data::new(create_schema()))
            .app_data(web::Data::new(repositories.clone()))
            .app_data(web::Data::new(usecases.clone()))
            .configure(configure_routes)
    });
    let bind_address = config.server.bind_address();
    server.bind(bind_address)?.run().await?;

    Ok(())
}
