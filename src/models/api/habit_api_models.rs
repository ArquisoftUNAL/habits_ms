use crate::models::{
    api::recurrence_api_models::*,
    database::{Habit, HabitRecurrence},
};
use crate::schema::habit;
use diesel::query_builder::AsChangeset;
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

    pub hab_is_yn: bool,

    pub hab_color: String,

    pub hab_units: String,

    pub usr_id: String,

    pub cat_id: Uuid,

    pub recurrences: Vec<HabitRecurrence>,
}

#[derive(Debug, Serialize)]
pub struct HabitRecurrencesAndData {
    pub hab_id: Uuid,

    pub hab_name: String,

    pub hab_description: String,

    pub hab_is_favorite: bool,

    pub hab_is_yn: bool,

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

    pub is_yn: bool,

    #[validate(length(min = 6, max = 6))]
    pub color: String,

    #[validate(length(min = 10, max = 10))]
    pub units: String,

    pub user_id: String,

    pub category: Uuid,
}

// Requests schemas
#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = habit)]
pub struct HabitUpdateSchema {
    #[validate(length(min = 1, max = 255))]
    #[diesel(column_name = "hab_name")]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 255))]
    #[diesel(column_name = "hab_description")]
    pub description: Option<String>,

    #[diesel(column_name = "hab_is_favorite")]
    pub is_favourite: Option<bool>,

    #[diesel(column_name = "hab_is_yn")]
    pub is_yn: Option<bool>,

    #[validate(length(min = 6, max = 6))]
    #[diesel(column_name = "hab_color")]
    pub color: Option<String>,

    #[validate(length(min = 10, max = 10))]
    #[diesel(column_name = "hab_units")]
    pub units: Option<String>,

    #[diesel(column_name = "usr_id")]
    pub user_id: Option<String>,

    #[diesel(column_name = "cat_id")]
    pub category: Option<Uuid>,
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
