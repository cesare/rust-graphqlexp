use super::{
    id::Id,
    servant::ServantId,
};

mod number;
pub use number::ProfileNumber;

pub type ProfileId = Id<Profile, String>;

pub struct Profile {
    pub id: ProfileId,
    pub servant_id: ServantId,
    pub number: ProfileNumber,
    pub text: String,
}
