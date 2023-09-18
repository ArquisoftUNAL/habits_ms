use crate::models::api::habit_api_models::HabitWithRecurrences;
use chrono::NaiveDate;
use serde_derive::Serialize;

// Embedded models
#[derive(Debug, Serialize)]
pub struct EventWithCount {
    pub date: NaiveDate,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct EventWithHabit {
    pub date: NaiveDate,
    pub habit: Vec<HabitWithRecurrences>,
}

// Response models
#[derive(Debug, Serialize)]
pub struct EventsHabitsMultipleQueryResponse {
    pub message: String,
    pub events: Vec<EventWithHabit>,
}

#[derive(Debug, Serialize)]
pub struct EventsCountMultipleQueryResponse {
    pub message: String,
    pub events: Vec<EventWithCount>,
}
