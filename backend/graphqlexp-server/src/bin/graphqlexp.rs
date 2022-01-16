use actix_web::{App, HttpServer, middleware::Logger};
use anyhow::Result;
use simplelog::{Config, LevelFilter, SimpleLogger};

fn initialize_logger() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, Config::default())?;
    Ok(())
}

#[actix_rt::main]
async fn main() -> Result<()> {
    initialize_logger()?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
    });
    server.bind("127.0.0.1:8080")?.run().await?;

    Ok(())
}
