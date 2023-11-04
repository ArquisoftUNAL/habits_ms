use crate::models::database::{HabFreqTypeEnum, Habit, HabitDataCollected};
use crate::schema::habit;
use diesel::query_builder::AsChangeset;
use serde_derive::{Deserialize, Serialize};

use bigdecimal::BigDecimal;
use uuid::Uuid;
use validator::Validate;

// Embedded models
#[derive(Debug, Serialize)]
pub struct HabitWithData {
    pub hab_id: Uuid,

    pub hab_name: String,

    pub hab_description: String,

    pub hab_is_favorite: bool,

    pub hab_is_yn: bool,

    pub hab_color: String,

    pub hab_units: String,

    pub hab_goal: BigDecimal,

    pub hab_freq_type: HabFreqTypeEnum,

    pub usr_id: String,

    pub cat_id: Uuid,

    pub data: Vec<HabitDataCollected>,
}

// Requests schemas
#[derive(Debug, Deserialize, Validate)]
pub struct HabitCreateSchema {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(length(min = 1, max = 255))]
    pub description: String,

    pub is_favorite: bool,

    pub is_yn: bool,

    #[validate(length(min = 6, max = 6))]
    pub color: String,

    #[validate(length(min = 1, max = 10))]
    pub units: String,

    #[validate(custom = "crate::validators::validate_bigdecimal")]
    pub goal: BigDecimal,

    pub frequency_type: HabFreqTypeEnum,

    pub category: Uuid,

    pub location: Option<String>,
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
    pub is_favorite: Option<bool>,

    #[diesel(column_name = "hab_is_yn")]
    pub is_yn: Option<bool>,

    #[validate(length(min = 6, max = 6))]
    #[diesel(column_name = "hab_color")]
    pub color: Option<String>,

    #[validate(length(min = 1, max = 10))]
    #[diesel(column_name = "hab_units")]
    pub units: Option<String>,

    #[validate(custom = "crate::validators::validate_bigdecimal")]
    #[diesel(column_name = "hab_goal")]
    pub goal: Option<BigDecimal>,

    #[diesel(column_name = "hab_freq_type")]
    pub frequency_type: Option<HabFreqTypeEnum>,

    #[diesel(column_name = "cat_id")]
    pub category: Option<Uuid>,

    #[diesel(column_name = "hab_location")]
    pub location: Option<String>,
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
pub struct HabitAndDataMultipleQueryResponse {
    pub message: String,

    pub habits: Vec<HabitWithData>,
}

#[derive(Debug, Serialize)]
pub struct HabitSingleQueryResponse {
    pub message: String,

    pub habit: Habit,
}

#[derive(Debug, Serialize)]
pub struct HabitAndDataSingleQueryResponse {
    pub message: String,

    pub habit: HabitWithData,
}
