// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "hab_freq_type_enum"))]
    pub struct HabFreqTypeEnum;
}

diesel::table! {
    category (cat_id) {
        cat_id -> Uuid,
        #[max_length = 45]
        cat_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HabFreqTypeEnum;

    habit (hab_id) {
        hab_id -> Uuid,
        #[max_length = 255]
        hab_name -> Varchar,
        #[max_length = 255]
        hab_description -> Varchar,
        hab_created_at -> Timestamp,
        hab_updated_at -> Timestamp,
        hab_is_yn -> Bool,
        hab_is_favorite -> Bool,
        #[max_length = 6]
        hab_color -> Varchar,
        #[max_length = 10]
        hab_units -> Varchar,
        hab_goal -> Numeric,
        hab_freq_type -> HabFreqTypeEnum,
        #[max_length = 24]
        usr_id -> Varchar,
        cat_id -> Uuid,
    }
}

diesel::table! {
    habit_data_collected (hab_dat_id) {
        hab_dat_id -> Uuid,
        hab_dat_amount -> Numeric,
        hab_dat_collected_at -> Date,
        hab_id -> Uuid,
    }
}

diesel::joinable!(habit -> category (cat_id));
diesel::joinable!(habit_data_collected -> habit (hab_id));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    habit,
    habit_data_collected,
);
