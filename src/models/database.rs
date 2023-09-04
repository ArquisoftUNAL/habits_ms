use bigdecimal::BigDecimal;
use chrono::prelude::*;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Queryable, Selectable, Insertable, Serialize, AsChangeset)]
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
    pub created_at: chrono::NaiveDateTime,

    #[diesel(column_name = "hab_updated_at")]
    pub updated_at: chrono::NaiveDateTime,

    #[diesel(column_name = "hab_is_favorite")]
    pub is_favorite: bool,

    #[diesel(column_name = "hab_type")]
    pub kind: String,

    #[diesel(column_name = "user_id")]
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Queryable, Selectable, Insertable, Serialize, AsChangeset)]
#[diesel(belongs_to(Habit, foreign_key = hab_id))]
#[diesel(table_name=crate::schema::habit_recurrency)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HabitRecurrency {
    #[diesel(column_name = "hab_rec_id")]
    pub id: Uuid,

    #[diesel(column_name = "hab_id")]
    pub habit_id: Uuid,

    #[diesel(column_name = "hab_rec_frequency_type")]
    pub frequency_type: String,

    #[diesel(column_name = "hab_rec_frequency_data")]
    pub frequency_data: NaiveDateTime,
}

#[derive(Debug, Deserialize, Queryable, Selectable, Insertable, Serialize, AsChangeset)]
#[diesel(belongs_to(HabitRecurrency, foreign_key = hab_rec_id))]
#[diesel(table_name=crate::schema::habit_data_collected)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HabitDataCollected {
    #[diesel(column_name = "hab_dat_id")]
    pub id: Uuid,

    #[diesel(column_name = "hab_dat_amount")]
    pub amount: BigDecimal,

    #[diesel(column_name = "hab_dat_collected_at")]
    pub collectedd_at: NaiveDateTime,

    #[diesel(column_name = "hab_rec_id")]
    pub habit_recurrency_id: Uuid,
}
