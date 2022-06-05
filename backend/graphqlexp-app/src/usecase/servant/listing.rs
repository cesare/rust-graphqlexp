use graphqlexp_adapter::models::servant::Servant;

pub struct ListingServants {
}

impl ListingServants {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(&self) -> Result<Vec<Servant>, Box<dyn std::error::Error>> {
        todo!()
    }
}
