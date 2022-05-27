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

impl From<ProfileRecord> for Profile {
    fn from(record: ProfileRecord) -> Self {
        Self {
            id: ProfileId::new(record.id),
            servant_id: ServantId::new(record.servant_id),
            position: ProfilePosition::new(record.position),
            text: record.text,
        }
    }
}
