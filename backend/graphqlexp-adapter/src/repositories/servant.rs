use anyhow::Result;
use async_trait::async_trait;
use sqlx::query_as;

pub use graphqlexp_kernel::{
    models::servant::{Servant, ServantId},
    repositories::servant::ServantRepository,
};
use super::Repository;
use crate::records::servant::ServantRecord;

#[async_trait]
impl ServantRepository for Repository<Servant> {
    async fn find(&self, id: ServantId) -> Result<Option<Servant>> {
        let pool = self.database.pool.clone();
        let statement = "
            select id, name, class_name, rarity, created_at, updated_at
            from servants
            where id = $1
            limit 1
        ";
        let result = query_as::<_, ServantRecord>(statement)
            .bind(id.value)
            .fetch_optional(&*pool)
            .await?;
        match result {
            Some(record) => Ok(Some(record.try_into()?)),
            None => Ok(None),
        }
    }
}
