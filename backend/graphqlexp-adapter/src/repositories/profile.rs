use async_trait::async_trait;
use sqlx::query_as;

use graphqlexp_kernel::models::{
    profile::{Profile, ProfileId},
    servant::ServantId
};
pub use graphqlexp_kernel::{
    repositories::profile::{NewProfile, ProfileRepository},
};

use crate::{
    repositories::Repository,
    records::profile::ProfileRecord,
    Error,
};

#[async_trait]
impl ProfileRepository for Repository<Profile> {
    type Error = Error;

    async fn find(&self, id: &ProfileId) -> Result<Option<Profile>, Error> {
        let pool = self.database.pool.clone();
        let statement = "
            select id, servant_id, position, text
            from profiles
            where id = $1
            limit 1
        ";
        let result = query_as::<_, ProfileRecord>(statement)
            .bind(&id.value)
            .fetch_optional(&*pool)
            .await?;

        let profile = result.map(|record| record.into());
        Ok(profile)
    }

    async fn list_for_servant(&self, servant_id: &ServantId) -> Result<Vec<Profile>, Error> {
        let pool = self.database.pool.clone();
        let statement = "
            select id, servant_id, position, text
            from profiles
            where servant_id = $1
            order by position
        ";
        let results = query_as::<_, ProfileRecord>(statement)
            .bind(&servant_id.value)
            .fetch_all(&*pool)
            .await?;

        let profiles = results.into_iter()
            .map(|record| record.into())
            .collect();
        Ok(profiles)
    }

    async fn list_for_servants(&self, ids: &[ServantId]) -> Result<Vec<Profile>, Error> {
        let pool = self.database.pool.clone();
        let statement = "
            select id, servant_id, position, text
            from profiles
            where servant_id = any($1)
        ";
        let raw_ids: Vec<String> = ids.iter().map(|id| id.value.to_owned()).collect();
        let results = query_as::<_, ProfileRecord>(statement)
            .bind(raw_ids)
            .fetch_all(&*pool)
            .await?;

        let profiles = results.into_iter().map(|record| record.into()).collect();
        Ok(profiles)
    }

    async fn register(&self, profile: NewProfile) -> Result<Profile, Error> {
        let pool = self.database.pool.clone();
        let statement = "
            insert into profiles (id, servant_id, position, text)
            values ($1, $2, $3, $4)
            returning id, servant_id, position, text
        ";
        let result = query_as::<_, ProfileRecord>(statement)
            .bind(cuid::cuid()?)
            .bind(profile.servant_id.value)
            .bind(profile.position.value())
            .bind(profile.text)
            .fetch_one(&*pool)
            .await?;

        let profile = result.into();
        Ok(profile)
    }
}
