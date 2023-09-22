use crate::models::database::HabitDataCollected;
use crate::schema::habit_data_collected;
use serde_derive::{Deserialize, Serialize};

use diesel::query_builder::AsChangeset;

use bigdecimal::BigDecimal;
use uuid::Uuid;

use validator::Validate;

// Input schemas
#[derive(Debug, Deserialize, Validate)]
pub struct HabitDataCreateSchema {
    #[validate(custom = "crate::validators::validate_bigdecimal")]
    pub amount: BigDecimal,

    #[validate(custom = "crate::validators::validate_habdata_collected_at")]
    pub collected_at: Option<chrono::NaiveDate>,

    // Optional for update only
    pub habit_id: Uuid,
}

#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = habit_data_collected)]
pub struct HabitDataUpdateSchema {
    #[validate(custom = "crate::validators::validate_bigdecimal")]
    #[diesel(column_name = "hab_dat_amount")]
    pub amount: BigDecimal,
}

// Response schemas
#[derive(Debug, Serialize)]
pub struct HabitDataCreateResponse {
    pub message: String,

    pub id: Uuid,

    pub habit_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct HabitDataUpdateDeleteResponse {
    pub message: String,

    pub habit_id: Uuid,
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
