use bigdecimal::BigDecimal;
use chrono::prelude::*;
use diesel::prelude::*;
use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Queryable, Selectable)]
#[diesel(table_name=crate::schema::habit)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Habit {
    #[diesel(column_name = "hab_id")]
    pub id: Uuid,

    #[diesel(column_name = "hab_name")]
    pub name: String,

    #[diesel(column_name = "hab_description")]
    pub description: String,

    #[diesel(column_name = "hab_created_at")]
    pub createdAt: chrono::NaiveDateTime,

    #[diesel(column_name = "hab_updated_at")]
    pub updatedAt: chrono::NaiveDateTime,

    #[diesel(column_name = "hab_is_favorite")]
    pub isFavorite: bool,

    #[diesel(column_name = "hab_type")]
    pub kind: String,

    #[diesel(column_name = "user_id")]
    pub userId: Uuid,
}

#[derive(Debug, Deserialize, Queryable, Associations, Selectable)]
#[diesel(belongs_to(Habit, foreign_key = hab_id))]
#[diesel(table_name=crate::schema::habit_recurrency)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HabitRecurrency {
    #[diesel(column_name = "hab_rec_id")]
    pub id: Uuid,

    #[diesel(column_name = "hab_id")]
    pub habitId: Uuid,

    #[diesel(column_name = "hab_rec_frequency_type")]
    pub frequencyType: String,

    #[diesel(column_name = "hab_rec_frequency_data")]
    pub frequencyData: NaiveDateTime,
}

#[derive(Debug, Deserialize, Queryable, Associations, Selectable)]
#[diesel(belongs_to(HabitRecurrency, foreign_key = hab_rec_id))]
#[diesel(table_name=crate::schema::habit_data_collected)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HabitDataCollected {
    #[diesel(column_name = "hab_dat_id")]
    pub id: Uuid,

    #[diesel(column_name = "hab_dat_amount")]
    pub amount: BigDecimal,

    #[diesel(column_name = "hab_collected_at")]
    pub collecteddAt: NaiveDateTime,

    #[diesel(column_name = "hab_rec_id")]
    pub habitRecurrencyId: Uuid,
}
