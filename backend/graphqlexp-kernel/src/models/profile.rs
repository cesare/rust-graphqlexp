use super::{
    id::{BelongsTo, Id, Identifiable},
    servant::{Servant, ServantId},
};

mod position;
pub use position::ProfilePosition;

pub type ProfileId = Id<Profile, String>;

#[derive(Clone)]
pub struct Profile {
    pub id: ProfileId,
    pub servant_id: ServantId,
    pub position: ProfilePosition,
    pub text: String,
}

impl Identifiable<Profile, String> for Profile {
    fn identifier(&self) -> &ProfileId {
        &self.id
    }
}

impl BelongsTo<Servant, i32> for Profile {
    fn parent_id(&self) -> &ServantId {
        &self.servant_id
    }
}
