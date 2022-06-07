pub mod map;
pub mod profile;

use graphqlexp_app::modules::UsecasesModule;

use profile::ServantProfilesLoader;

pub struct Loaders {
    pub servant_profiles_loader: ServantProfilesLoader,
}

impl Loaders {
    pub fn new(usecases: &UsecasesModule) -> Self {
        Self {
            servant_profiles_loader: profile::servant_profiles_loader(usecases),
        }
    }
}
