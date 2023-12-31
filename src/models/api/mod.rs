pub mod category_api_models;
pub mod data_api_models;
pub mod events_api_models;
pub mod habit_api_models;

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
    pub data_page: Option<i64>,
    pub data_per_page: Option<i64>,
    pub events_limit: Option<i64>,
}

// Authentication data matcher
#[derive(Debug, Clone, Deserialize)]
pub enum AuthRole {
    User,
    Guest,
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub requester_id: String,

    pub role: AuthRole,
}

// Date query params
#[derive(Debug, Deserialize)]
pub struct DateParams {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

// Data include params matcher, we can easily tell a handler to include or not certain data in response
#[derive(Debug, Deserialize)]

pub struct DataIncludeParams {
    pub include_habits: Option<bool>,
    pub include_data: Option<bool>,
}

impl Default for DataIncludeParams {
    fn default() -> Self {
        DataIncludeParams {
            include_habits: Some(false),
            include_data: Some(false),
        }
    }
}
