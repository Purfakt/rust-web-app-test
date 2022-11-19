use crate::model::expenditure::{Category, Cost, Expenditure, Ownership};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenditureDocument {
    pub _id: bson::Uuid,
    pub description: String,
    pub category: Option<Category>,
    pub cost: Cost,
    pub ownership: Ownership,
}

impl From<&Expenditure> for ExpenditureDocument {
    fn from(value: &Expenditure) -> Self {
        let bson_id = bson::Uuid::from_bytes(*value.id.as_bytes());

        ExpenditureDocument {
            _id: bson_id,
            description: value.description.clone(),
            category: value.category.clone(),
            cost: value.cost,
            ownership: value.ownership.clone(),
        }
    }
}

impl From<&ExpenditureDocument> for Expenditure {
    fn from(value: &ExpenditureDocument) -> Self {
        let id = uuid::Uuid::from_bytes(value._id.bytes());

        Expenditure {
            id,
            description: value.description.clone(),
            category: value.category.clone(),
            cost: value.cost,
            ownership: value.ownership.clone(),
        }
    }
}
