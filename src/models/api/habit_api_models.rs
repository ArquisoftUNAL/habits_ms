use crate::models::database::{Habit, HabitRecurrency};
use serde_derive::{Deserialize, Serialize};

// use bigdecimal::BigDecimal;
use uuid::Uuid;

use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct HabitCreateSchema {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(length(min = 1, max = 255))]
    pub description: String,

    pub is_favourite: bool,

    pub kind: String,

    #[validate(length(min = 6, max = 6))]
    pub color: String,

    pub units: String,

    pub user_id: String,

    pub category: Uuid,
}

#[derive(Debug, Serialize)]
pub struct HabitCreateResponse {
    pub message: String,

    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct HabitMultipleQueryResponse {
    pub message: String,

    pub habits: Vec<Habit>,
}

#[derive(Debug, Serialize)]
pub struct HabitSingleQueryResponse {
    pub message: String,

    pub habit: Habit,
}

#[derive(Debug, Serialize)]
pub struct HabitsAndRecurrencesQueryResponse {
    pub message: String,

    pub habits: Vec<(Habit, Vec<HabitRecurrency>)>,
}
