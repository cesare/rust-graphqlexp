use std::sync::Arc;

use sqlx::postgres::{PgPool, PgPoolOptions};

use super::modules::RepositoriesModuleConfig;
use crate::Error;

#[derive(Clone)]
pub struct Database {
    pub(crate) pool: Arc<PgPool>,
}

impl Database {
    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self, Error> {
        let url = config.database_url();
        let pool = PgPoolOptions::new()
            .connect(&url)
            .await?;

        let db = Self {
            pool: Arc::new(pool),
        };
        Ok(db)
    }
}
