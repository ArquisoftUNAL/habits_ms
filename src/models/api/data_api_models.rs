use crate::models::database::HabitDataCollected;
use crate::schema::habit_data_collected;
use serde_derive::{Deserialize, Serialize};

use diesel::query_builder::AsChangeset;

use bigdecimal::BigDecimal;
use uuid::Uuid;

use validator::Validate;

// Input schemas
#[derive(Debug, Deserialize, Validate)]
pub struct HabitDataSchema {
    #[validate(custom = "crate::validators::validate_bigdecimal")]
    pub amount: BigDecimal,

    pub collected_at: Option<chrono::NaiveDate>,

    // Optional for update only
    pub recurrence_id: Uuid,
}

#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = habit_data_collected)]
pub struct HabitDataUpdateSchema {
    #[validate(custom = "crate::validators::validate_bigdecimal")]
    #[diesel(column_name = "hab_dat_amount")]
    pub amount: BigDecimal,

    // Optional for update only
    #[diesel(column_name = "hab_rec_id")]
    pub recurrence_id: Uuid,
}

// Response schemas
#[derive(Debug, Serialize)]
pub struct HabitDataCreateResponse {
    pub message: String,

    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct HabitDataMultipleQueryResponse {
    pub message: String,

    pub habit_data: Vec<HabitDataCollected>,
}

#[derive(Debug, Serialize)]
pub struct HabitDataSingleQueryResponse {
    pub message: String,

    pub habit_data: HabitDataCollected,
}

// Custom validators
