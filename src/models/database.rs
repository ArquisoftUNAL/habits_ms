use bigdecimal::BigDecimal;
use chrono::prelude::*;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::RecDataType"]
pub enum RecDataEnum {
    daily,
    weekly,
    weekly2,
    montly,
    montly2,
}

#[derive(
    Debug, Deserialize, Queryable, Selectable, Insertable, Serialize, AsChangeset, Identifiable,
)]
#[diesel(primary_key(cat_id))]
#[diesel(table_name=crate::schema::category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub cat_id: Uuid,

    pub cat_name: String,
}

#[derive(
    Debug,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    AsChangeset,
    Identifiable,
    Associations,
)]
#[diesel(belongs_to(Category, foreign_key = cat_id))]
#[diesel(primary_key(hab_id))]
#[diesel(table_name=crate::schema::habit)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Habit {
    pub hab_id: Uuid,

    pub hab_name: String,

    pub hab_description: String,

    pub hab_created_at: chrono::NaiveDateTime,

    pub hab_updated_at: chrono::NaiveDateTime,

    pub hab_is_favorite: bool,

    pub hab_is_yn: bool,

    pub hab_color: String,

    pub hab_units: String,

    pub usr_id: String,

    pub cat_id: Uuid,
}

#[derive(
    Debug,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    AsChangeset,
    Associations,
    Identifiable,
)]
#[diesel(belongs_to(Habit, foreign_key = hab_id))]
#[diesel(primary_key(hab_rec_id))]
#[diesel(table_name=crate::schema::habit_recurrency)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HabitRecurrency {
    pub hab_rec_id: Uuid,

    pub hab_id: Uuid,

    pub hab_rec_freq_type: RecDataEnum,

    pub hab_rec_goal: BigDecimal,

    pub hab_rec_freq_data: NaiveDate,
}

#[derive(
    Debug,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    AsChangeset,
    Identifiable,
    Associations,
)]
#[diesel(belongs_to(HabitRecurrency, foreign_key = hab_rec_id))]
#[diesel(primary_key(hab_dat_id))]
#[diesel(table_name=crate::schema::habit_data_collected)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HabitDataCollected {
    pub hab_dat_id: Uuid,

    pub hab_dat_amount: BigDecimal,

    pub hab_dat_collected_at: NaiveDateTime,

    pub hab_rec_id: Uuid,
}
