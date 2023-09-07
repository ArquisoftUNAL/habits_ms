use crate::models::database::HabitDataCollected;
use serde_derive::{Deserialize, Serialize};

use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct HabitDataRequest {
    pub amount: BigDecimal,

    // Optional for update only
    pub recurrency_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct HabitDataMultipleQuery {
    pub status: i16,

    pub habits: Vec<HabitDataCollected>,
}
