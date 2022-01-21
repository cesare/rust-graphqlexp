use std::sync::Arc;

use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

use super::modules::RepositoriesModuleConfig;

#[derive(Clone)]
pub struct Database {
    #[allow(dead_code)]
    pub(crate) pool: Arc<PgPool>,
}

impl Database {
    pub async fn create(config: &dyn RepositoriesModuleConfig) -> Result<Self> {
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
