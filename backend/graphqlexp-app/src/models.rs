pub use graphqlexp_adapter::models::{
    BelongsTo, Id, Identifiable,
};

pub mod profile {
    pub use graphqlexp_adapter::models::profile::{
        Profile, ProfileId, ProfilePosition,
    };
}

pub mod servant {
    pub use graphqlexp_adapter::models::servant::{
        Class, Name, Rarity, Servant, ServantId,
    };
}
