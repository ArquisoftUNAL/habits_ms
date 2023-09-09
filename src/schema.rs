// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rec_data_type"))]
    pub struct RecDataType;
}

diesel::table! {
    category (cat_id) {
        cat_id -> Uuid,
        #[max_length = 45]
        cat_name -> Varchar,
    }
}

diesel::table! {
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
        #[max_length = 24]
        usr_id -> Varchar,
        cat_id -> Uuid,
    }
}

diesel::table! {
    habit_data_collected (hab_dat_id) {
        hab_dat_id -> Uuid,
        hab_dat_amount -> Numeric,
        hab_dat_collected_at -> Timestamp,
        hab_rec_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RecDataType;

    habit_recurrency (hab_rec_id) {
        hab_rec_id -> Uuid,
        hab_rec_freq_type -> RecDataType,
        hab_rec_freq_data -> Date,
        hab_id -> Uuid,
    }
}

diesel::joinable!(habit -> category (cat_id));
diesel::joinable!(habit_data_collected -> habit_recurrency (hab_rec_id));
diesel::joinable!(habit_recurrency -> habit (hab_id));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    habit,
    habit_data_collected,
    habit_recurrency,
);
