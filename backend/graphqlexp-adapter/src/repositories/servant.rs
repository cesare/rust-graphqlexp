use async_trait::async_trait;
use sqlx::query_as;

use graphqlexp_kernel::{
    models::servant::{Servant, ServantId},
};
pub use graphqlexp_kernel::{
    repositories::servant::{NewServant, ServantRepository},
};
use super::Repository;
use crate::records::servant::ServantRecord;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

    async fn list(&self) -> Result<Vec<Servant>> {
        let pool = self.database.pool.clone();
        let statement = "
            select id, name, class_name, rarity, created_at, updated_at
            from servants
        ";
        let results = query_as::<_, ServantRecord>(statement)
            .fetch_all(&*pool)
            .await?;

        let mut servants: Vec<Servant> = vec![];
        for record in results {
            let servant = record.try_into()?;
            servants.push(servant);
        }
        Ok(servants)
    }

    async fn register(&self, servant: NewServant) -> Result<Servant> {
        let pool = self.database.pool.clone();
        let statement = "
            insert into servants (id, name, class_name, rarity)
            values ($1, $2, $3, $4)
            returning id, name, class_name, rarity, created_at, updated_at
        ";
        let result = query_as::<_, ServantRecord>(statement)
            .bind(cuid::cuid()?)
            .bind(servant.name)
            .bind(servant.class.to_string())
            .bind(servant.rarity.value())
            .fetch_one(&*pool)
            .await?;
        let servant = result.try_into()?;
        Ok(servant)
    }
}
