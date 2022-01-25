use std::path::PathBuf;

use actix_web::{App, HttpServer, middleware::Logger, web};
use anyhow::Result;
use simplelog::{Config, LevelFilter, SimpleLogger};
use structopt::StructOpt;

use graphqlexp_server::{
    config::{GraphqlexpConfig},
    routes::graphql,
    schema::create_schema,
};

#[derive(StructOpt)]
#[structopt(name = "graphqlexp")]
struct Args {
    #[structopt(short, long, parse(from_os_str))]
    config_file: PathBuf,
}

fn initialize_logger() -> Result<()> {
    SimpleLogger::init(LevelFilter::Debug, Config::default())?;
    Ok(())
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let config = GraphqlexpConfig::load(&args.config_file).await?;

    initialize_logger()?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .app_data(web::Data::new(create_schema()))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
    });
    let bind_address = config.server.bind_address();
    server.bind(bind_address)?.run().await?;

    Ok(())
}
