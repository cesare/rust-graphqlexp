use actix_web::middleware::Logger;
use anyhow::Result;
use simplelog::{Config, LevelFilter, SimpleLogger};

pub fn initialize_logger() -> Result<()> {
    SimpleLogger::init(LevelFilter::Debug, Config::default())?;
    Ok(())
}

pub fn default_logger() -> Logger {
    Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
}
