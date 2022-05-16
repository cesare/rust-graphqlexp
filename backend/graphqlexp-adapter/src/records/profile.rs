use sqlx::FromRow;

use graphqlexp_kernel::models::{
    profile::{Profile, ProfileId, ProfilePosition},
    servant::ServantId
};

#[derive(FromRow)]
pub(crate) struct ProfileRecord {
    pub id: String,
    pub servant_id: String,
    pub position: i32,
    pub text: String,
}

impl TryFrom<ProfileRecord> for Profile {
    type Error = crate::Error;

    fn try_from(record: ProfileRecord) -> Result<Self, Self::Error> {
        let profile = Self {
            id: ProfileId::new(record.id),
            servant_id: ServantId::new(record.servant_id),
            position: ProfilePosition::create(record.position)?,
            text: record.text,
        };
        Ok(profile)
    }
}
