use std::path::PathBuf;

use actix_web::{App, HttpServer, middleware::Logger};
use anyhow::Result;
use serde::Deserialize;
use simplelog::{Config, LevelFilter, SimpleLogger};
use structopt::StructOpt;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(StructOpt)]
#[structopt(name = "graphqlexp")]
struct Args {
    #[structopt(short, long, parse(from_os_str))]
    config_file: PathBuf,
}

impl Args {
    async fn load_config(&self) -> Result<GraphqlexpConfig> {
        let mut file = File::open(&self.config_file).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Deserialize)]
struct GraphqlexpConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    bind: String,
    port: u32,
}

impl ServerConfig {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}

#[derive(Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u32,
    database: String,
    username: String,
    password: String,
}

fn initialize_logger() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, Config::default())?;
    Ok(())
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let config = args.load_config().await?;

    initialize_logger()?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
    });
    let bind_address = config.server.bind_address();
    server.bind(bind_address)?.run().await?;

    Ok(())
}
