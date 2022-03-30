use std::str::FromStr;

use chrono::{DateTime, Local};
use sqlx::FromRow;

use graphqlexp_kernel::models::servant::{Class, Rarity, Servant, ServantId};

#[derive(FromRow)]
pub struct ServantRecord {
    pub id: String,
    pub name: String,
    pub class_name: String,
    pub rarity: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl TryFrom<ServantRecord> for Servant {
    type Error = anyhow::Error;

    fn try_from(record: ServantRecord) -> Result<Self, Self::Error> {
        let servant = Self {
            id: ServantId::new(record.id),
            name: record.name,
            class: Class::from_str(&record.class_name)?,
            rarity: Rarity::create(record.rarity)?,
            created_at: record.created_at,
            updated_at: record.updated_at,
        };
        Ok(servant)
    }
}
