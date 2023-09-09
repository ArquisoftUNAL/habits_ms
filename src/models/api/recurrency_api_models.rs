use crate::models::database::{HabitDataCollected, HabitRecurrency, RecDataEnum};
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use validator::Validate;

// Embedded models
#[derive(Debug, Serialize)]
pub struct RecurrenceWithData {
    pub hab_rec_id: Uuid,

    pub hab_id: Uuid,

    pub hab_rec_freq_type: RecDataEnum,

    pub hab_rec_freq_data: NaiveDate,

    pub data: Vec<HabitDataCollected>,
}

// Requests schemas
#[derive(Debug, Deserialize, Validate)]
pub struct RecurrencyCreateSchema {
    pub frequency_type: RecDataEnum,

    pub frequency_data: NaiveDate,

    // Optional for update only
    pub habit_id: Uuid,
}

// Responses
#[derive(Debug, Serialize)]
pub struct RecurrencesMultipleQueryResponse {
    pub message: String,

    pub recurrences: Vec<HabitRecurrency>,
}

#[derive(Debug, Serialize)]
pub struct RecurrenceCreateResponse {
    pub message: String,

    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct RecurrencesWithDataMultipleQueryResponse {
    pub message: String,

    pub recurrences: Vec<RecurrenceWithData>,
}

#[derive(Debug, Serialize)]
pub struct RecurrencesSingleQueryResponse {
    pub message: String,

    pub recurrence: HabitRecurrency,
}

#[derive(Debug, Serialize)]
pub struct RecurrencesWithDataSingleQueryResponse {
    pub message: String,

    pub recurrence: RecurrenceWithData,
}
