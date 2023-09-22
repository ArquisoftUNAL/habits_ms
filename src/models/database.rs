use bigdecimal::BigDecimal;
use chrono::prelude::*;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize, Serialize, Clone, Copy)]
#[ExistingTypePath = "crate::schema::sql_types::HabFreqTypeEnum"]
pub enum HabFreqTypeEnum {
    daily,
    daily2,
    weekly,
    weekly2,
    monthly,
    monthly2,
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
    Clone,
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

    pub hab_goal: BigDecimal,

    pub hab_freq_type: HabFreqTypeEnum,

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
    Identifiable,
    Associations,
    Clone,
)]
#[diesel(belongs_to(Habit, foreign_key = hab_id))]
#[diesel(primary_key(hab_dat_id))]
#[diesel(table_name=crate::schema::habit_data_collected)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HabitDataCollected {
    pub hab_dat_id: Uuid,

    pub hab_dat_amount: BigDecimal,

    pub hab_dat_collected_at: NaiveDate,

    pub hab_id: Uuid,
}
