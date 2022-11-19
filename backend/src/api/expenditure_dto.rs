use crate::model::expenditure::{Category, Cost, Expenditure, Ownership};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenditureRead {
    pub id: uuid::Uuid,
    pub description: String,
    pub category: Option<Category>,
    pub cost: Cost,
    pub ownership: Ownership,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenditureCreate {
    pub description: String,
    pub category: Option<Category>,
    pub cost: Cost,
    pub ownership: Ownership,
}

impl ExpenditureRead {
    pub fn from_model(model: &Expenditure) -> Self {
        ExpenditureRead {
            id: model.id,
            description: model.description.clone(),
            category: model.category.clone(),
            cost: model.cost,
            ownership: model.ownership.clone(),
        }
    }
}

impl ExpenditureCreate {
    pub fn into_model(self, id: uuid::Uuid) -> Expenditure {
        Expenditure {
            id,
            description: self.description,
            category: self.category,
            cost: self.cost,
            ownership: self.ownership,
        }
    }
}
