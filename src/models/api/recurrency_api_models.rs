use crate::models::database::HabitRecurrency;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RecurrencesMultipleQuery {
    pub status: i16,

    pub habits: Vec<HabitRecurrency>,
}

#[derive(Debug, Deserialize)]
pub struct RecurrencyCreateRequest {
    pub frequency_type: String,

    pub frequency_data: NaiveDateTime,

    // Optional for update only
    pub habit_id: Uuid,
}
