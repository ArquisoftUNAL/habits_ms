pub mod category_api_models;
pub mod data_api_models;
pub mod habit_api_models;
pub mod recurrency_api_models;

use serde_derive::Serialize;

// General purpose response for common requests
#[derive(Debug, Serialize)]
pub struct GeneralResponse {
    pub message: String,
}
