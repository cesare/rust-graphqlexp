use super::{
    id::{Id, Identifiable},
    servant::ServantId,
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
