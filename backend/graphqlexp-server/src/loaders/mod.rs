pub mod profile;

use graphqlexp_app::modules::RepositoriesModule;

use profile::ServantProfilesLoader;

pub struct Loaders {
    pub servant_profiles_loader: ServantProfilesLoader,
}

impl Loaders {
    pub fn new(repositories: &RepositoriesModule) -> Self {
        Self {
            servant_profiles_loader: profile::servant_profiles_loader(repositories),
        }
    }
}
