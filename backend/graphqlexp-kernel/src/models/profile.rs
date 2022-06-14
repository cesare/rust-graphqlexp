use super::{
    id::{BelongsTo, Id, Identifiable},
    servant::{Servant, ServantId},
};

mod position;
pub use position::ProfilePosition;
mod text;
pub use text::ProfileText;

pub type ProfileId = Id<Profile>;

#[derive(Clone)]
pub struct Profile {
    pub id: ProfileId,
    pub servant_id: ServantId,
    pub position: ProfilePosition,
    pub text: String,
}

impl Identifiable<Profile> for Profile {
    fn identifier(&self) -> &ProfileId {
        &self.id
    }
}

impl BelongsTo<Servant> for Profile {
    fn parent_id(&self) -> &ServantId {
        &self.servant_id
    }
}
