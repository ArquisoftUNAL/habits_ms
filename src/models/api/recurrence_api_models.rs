use crate::models::database::{HabitDataCollected, HabitRecurrence, RecDataEnum};
use crate::schema::habit_recurrence;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::query_builder::AsChangeset;
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
pub struct RecurrenceCreateSchema {
    pub frequency_type: RecDataEnum,

    pub frequency_data: NaiveDate,

    #[validate(custom = "crate::validators::validate_bigdecimal")]
    pub goal: BigDecimal,

    // Optional for update only
    pub habit_id: Uuid,
}

#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = habit_recurrence)]
pub struct RecurrenceUpdateSchema {
    #[diesel(column_name = "hab_rec_freq_type")]
    pub frequency_type: Option<RecDataEnum>,

    #[diesel(column_name = "hab_rec_freq_data")]
    pub frequency_data: Option<NaiveDate>,

    #[validate(custom = "crate::validators::validate_bigdecimal")]
    #[diesel(column_name = "hab_rec_goal")]
    pub goal: Option<BigDecimal>,

    // Optional for update only
    #[diesel(column_name = "hab_id")]
    pub habit_id: Option<Uuid>,
}

// Responses
#[derive(Debug, Serialize)]
pub struct RecurrencesMultipleQueryResponse {
    pub message: String,

    pub recurrences: Vec<HabitRecurrence>,
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

    pub recurrence: HabitRecurrence,
}

#[derive(Debug, Serialize)]
pub struct RecurrencesWithDataSingleQueryResponse {
    pub message: String,

    pub recurrence: RecurrenceWithData,
}
