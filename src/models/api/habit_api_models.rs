use crate::models::database::Habit;
use serde_derive::{Deserialize, Serialize};

// use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct HabitMultipleQueryResponse {
    pub status: i16,

    pub habits: Vec<Habit>,
}

#[derive(Debug, Deserialize)]
pub struct HabitCreateRequest {
    pub name: String,

    pub description: String,

    pub is_favourite: bool,

    pub kind: String,

    pub color: String,

    pub units: String,

    pub user_id: String,

    pub category: Uuid,
}

#[derive(Debug, Serialize)]
pub struct HabitCreateResponse {
    pub status: i16,

    pub message: String,

    pub hab_id: Uuid,
}
