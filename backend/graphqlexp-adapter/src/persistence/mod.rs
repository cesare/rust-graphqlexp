use std::env;
use std::sync::Arc;

use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone)]
pub struct Database {
    #[allow(dead_code)]
    pub(crate) pool: Arc<PgPool>,
}

impl Database {
    pub async fn create() -> Result<Self> {
        let url = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .connect(&url)
            .await?;

        let db = Self {
            pool: Arc::new(pool),
        };
        Ok(db)
    }
}
