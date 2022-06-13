pub use graphqlexp_kernel::models::id::{
    BelongsTo, Id, Identifiable,
};

pub mod profile {
    pub use graphqlexp_kernel::models::profile::{
        Profile, ProfileId, ProfilePosition,
    };
}

pub mod servant {
    pub use graphqlexp_kernel::models::servant::{
        Class, Name, Rarity, Servant, ServantId,
    };
}
