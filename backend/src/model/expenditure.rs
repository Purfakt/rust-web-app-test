use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Periodicity {
    Monthly,
    Bimonthly,
    Trimestrial,
    Quarter,
    Semestrial,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Ownership {
    Common,
    Personal(User),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Cost {
    pub amount: f32,
    pub periodicity: Periodicity,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Expenditure {
    pub id: Uuid,
    pub description: String,
    pub category: Option<Category>,
    pub cost: Cost,
    pub ownership: Ownership,
}
