use crate::models::database::HabitDataCollected;
use serde_derive::{Deserialize, Serialize};

use bigdecimal::BigDecimal;
use uuid::Uuid;

use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct HabitDataSchema {
    #[validate(custom = "crate::validators::validate_bigdecimal")]
    pub amount: BigDecimal,

    // Optional for update only
    pub recurrency_id: Uuid,
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
