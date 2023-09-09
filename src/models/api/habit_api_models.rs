use crate::models::{
    api::recurrency_api_models::*,
    database::{Habit, HabitRecurrency},
};
use serde_derive::{Deserialize, Serialize};

// use bigdecimal::BigDecimal;
use uuid::Uuid;

use validator::Validate;

// Embedded models
#[derive(Debug, Serialize)]
pub struct HabitWithRecurrences {
    pub hab_id: Uuid,

    pub hab_name: String,

    pub hab_description: String,

    pub hab_is_favorite: bool,

    pub hab_type: String,

    pub hab_color: String,

    pub hab_units: String,

    pub usr_id: String,

    pub cat_id: Uuid,

    pub recurrences: Vec<HabitRecurrency>,
}

#[derive(Debug, Serialize)]
pub struct HabitRecurrencesAndData {
    pub hab_id: Uuid,

    pub hab_name: String,

    pub hab_description: String,

    pub hab_is_favorite: bool,

    pub hab_type: String,

    pub hab_color: String,

    pub hab_units: String,

    pub usr_id: String,

    pub cat_id: Uuid,

    pub recurrences: Vec<RecurrenceWithData>,
}

// Requests schemas
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

// Responses
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
pub struct HabitAndRecurrencesMultipleQueryResponse {
    pub message: String,

    pub habits: Vec<HabitWithRecurrences>,
}

#[derive(Debug, Serialize)]
pub struct HabitSingleQueryResponse {
    pub message: String,

    pub habit: Habit,
}

#[derive(Debug, Serialize)]
pub struct HabitAndRecurrencesSingleQueryResponse {
    pub message: String,

    pub habit: Habit,
}

#[derive(Debug, Serialize)]
pub struct HabitsAndRecurrencesAndDataQueryResponse {
    pub message: String,

    pub habits: Vec<HabitRecurrencesAndData>,
}