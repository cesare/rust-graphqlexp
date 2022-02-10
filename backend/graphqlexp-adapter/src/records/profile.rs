use sqlx::FromRow;

use graphqlexp_kernel::models::{
    profile::{Profile, ProfileId, ProfileNumber},
    servant::ServantId
};

#[derive(FromRow)]
pub(crate) struct ProfileRecord {
    pub id: String,
    pub servant_id: i32,
    pub number: i32,
    pub text: String,
}

impl TryFrom<ProfileRecord> for Profile {
    type Error = anyhow::Error;

    fn try_from(record: ProfileRecord) -> Result<Self, Self::Error> {
        let profile = Self {
            id: ProfileId::new(record.id),
            servant_id: ServantId::new(record.servant_id),
            number: ProfileNumber::create(record.number)?,
            text: record.text,
        };
        Ok(profile)
    }
}
