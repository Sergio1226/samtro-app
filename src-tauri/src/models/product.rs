use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    pub code: String,
    pub name: String,
    pub category: String,
    pub price: i32,
    pub stock: i32,
    pub minimum: i32,
    pub description: String,
    pub active: bool,
}