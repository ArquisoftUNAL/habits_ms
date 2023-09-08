use crate::models::database::HabitRecurrency;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use validator::Validate;

#[derive(Debug, Serialize)]
pub struct RecurrencesMultipleQueryResponse {
    pub message: String,

    pub habits: Vec<HabitRecurrency>,
}

#[derive(Debug, Serialize)]
pub struct RecurrencesSingleQueryResponse {
    pub message: String,

    pub habits: HabitRecurrency,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecurrencyCreateSchema {
    pub frequency_type: String,

    pub frequency_data: NaiveDateTime,

    // Optional for update only
    pub habit_id: Uuid,
}
