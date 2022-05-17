use actix_web::middleware::Logger;
use simplelog::{Config, LevelFilter, SimpleLogger};

use crate::GraphqlexpError;

pub fn initialize_logger() -> Result<(), GraphqlexpError> {
    SimpleLogger::init(LevelFilter::Debug, Config::default())?;
    Ok(())
}

pub fn default_logger() -> Logger {
    Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
}
