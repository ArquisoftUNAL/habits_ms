use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct GeneralResponse {
    pub status: i16,

    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct HabitCreateRequest {
    pub name: String,

    pub description: String,

    pub isFavorite: bool,

    pub kind: String,

    pub userId: Uuid,
}
