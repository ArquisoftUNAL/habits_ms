pub mod category_api_models;
pub mod data_api_models;
pub mod habit_api_models;
pub mod recurrence_api_models;

use serde_derive::{Deserialize, Serialize};

// General purpose response for common requests
#[derive(Debug, Serialize)]
pub struct GeneralResponse {
    pub message: String,
}

// Common query params matcher
#[derive(Debug, Deserialize)]
pub struct RangeParams {
    pub categories_page: Option<i64>,
    pub categories_per_page: Option<i64>,
    pub habits_page: Option<i64>,
    pub habits_per_page: Option<i64>,
    pub recurrences_page: Option<i64>,
    pub recurrences_per_page: Option<i64>,
    pub data_page: Option<i64>,
    pub data_per_page: Option<i64>,
}

// Data include params matcher, we can easily tell a handler to include or not certain data in response
#[derive(Debug, Deserialize)]

pub struct DataIncludeParams {
    pub include_habits: Option<bool>,
    pub include_recurrences: Option<bool>,
    pub include_data: Option<bool>,
}

impl Default for DataIncludeParams {
    fn default() -> Self {
        DataIncludeParams {
            include_habits: Some(false),
            include_recurrences: Some(false),
            include_data: Some(false),
        }
    }
}
