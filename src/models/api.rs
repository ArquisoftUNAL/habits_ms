use crate::models::database::*;
use bigdecimal::BigDecimal;
use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct GeneralResponse {
    pub status: i16,

    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct HabitMultipleQuery {
    pub status: i16,

    pub habits: Vec<Habit>,
}

#[derive(Debug, Serialize)]
pub struct RecurrencesMultipleQuery {
    pub status: i16,

    pub habits: Vec<HabitRecurrency>,
}

#[derive(Debug, Serialize)]
pub struct HabitDataMultipleQuery {
    pub status: i16,

    pub habits: Vec<HabitDataCollected>,
}

#[derive(Debug, Deserialize)]
pub struct HabitCreateRequest {
    pub name: String,

    pub description: String,

    pub is_favourite: bool,

    pub kind: String,

    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct RecurrencyCreateRequest {
    pub frequency_type: String,

    pub frequency_data: NaiveDateTime,

    // Optional for update only
    pub habit_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct HabitDataRequest {
    pub amount: BigDecimal,

    // Optional for update only
    pub recurrency_id: Uuid,
}
