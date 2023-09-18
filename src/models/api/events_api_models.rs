use crate::models::database::{Habit, HabitRecurrence};
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
    pub habit: Habit,
    pub recurrence: HabitRecurrence,
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
