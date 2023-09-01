use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use schema::{habit, habit_data_collected, habit_recurrency};

#[derive(Debug, Deserialize, Queryable, Selectable)]
#[diesel(table_name="habit")]
pub struct Habit {
    pub id: String,
    pub name: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
    pub isFavorite: bool,
    pub kind: String
}

#[derive(Debug, Deserialize, Queryable, Associations, Identifiable, Selectable)]
#[diesel(belongs_to(Habit, foreign_key = habitId))]
#[diesel(table_name="habit_recurrency")]
pub struct HabitRecurrency {
    pub id: String,
    pub habitId : String,
    pub frequencyType : String,
    pub frequencyData : DateTime<Utc>,
}

#[derive(Debug, Deserialize, Queryable, Associations, Identifiable, Selectable)]
#[diesel(belongs_to(HabitRecurrency, foreign_key = habitRecurrencyId))]
#[diesel(table_name="habit_data_collected")]
pub struct HabitDataCollected {
    pub id : String,
    pub data : Double,
    pub habitRecurrencyId : String,
    pub collecteddAt : DateTime<Utc>,
}