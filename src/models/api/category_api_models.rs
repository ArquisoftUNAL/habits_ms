use crate::models::database::Category;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Category specific API
#[derive(Debug, Deserialize, Validate)]
pub struct CategoryCreateSchema {
    #[validate(length(min = 1, max = 45))]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CategoryCreateResponse {
    pub message: String,

    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CategoryMultipleQueryResponse {
    pub message: String,

    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize)]
pub struct CategorySingleQueryResponse {
    pub message: String,

    pub category: Category,
}
