use anyhow::Result;
use async_trait::async_trait;
use sqlx::query_as;

pub use graphqlexp_kernel::{
    models::{
        profile::{Profile, ProfileId, ProfileNumber},
        servant::ServantId
    },
    repositories::profile::{NewProfile, ProfileRepository},
};
use super::Repository;
use crate::records::profile::ProfileRecord;

#[async_trait]
impl ProfileRepository for Repository<Profile> {
    async fn find(&self, id: &ProfileId) -> Result<Option<Profile>> {
        let pool = self.database.pool.clone();
        let statement = "
            select id, servant_id, number, text
            from profiles
            where id = $1
            limit 1
        ";
        let result = query_as::<_, ProfileRecord>(statement)
            .bind(&id.value)
            .fetch_optional(&*pool)
            .await?;
        match result {
            Some(record) => Ok(Some(record.try_into()?)),
            _ => Ok(None),
        }
    }

    async fn list_for_servant(&self, _servant_id: &ServantId) -> Result<Vec<Profile>> {
        todo!()
    }

    async fn register(&self, _profile: NewProfile) -> Result<Profile> {
        todo!()
    }
}
