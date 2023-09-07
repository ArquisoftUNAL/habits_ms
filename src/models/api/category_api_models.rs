use crate::models::database::Category;
use serde_derive::{Deserialize, Serialize};

// Category specific API
#[derive(Debug, Deserialize)]
pub struct CategoryCreateSchema {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CategoryMultipleQueryResponse {
    pub status: i16,

    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize)]
pub struct CategorySingleQueryResponse {
    pub status: i16,

    pub category: Category,
}
