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

impl From<ServantRecord> for Servant {
    fn from(record: ServantRecord) -> Self {
        Self {
            id: ServantId::new(record.id),
            name: record.name,
            class: Class::from(record.class_name.as_str()),
            rarity: Rarity::new(record.rarity),
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}
