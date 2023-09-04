// @generated automatically by Diesel CLI.

diesel::table! {
    habit (hab_id) {
        hab_id -> Uuid,
        #[max_length = 255]
        hab_name -> Varchar,
        #[max_length = 255]
        hab_description -> Varchar,
        hab_created_at -> Timestamp,
        hab_updated_at -> Timestamp,
        #[max_length = 10]
        hab_type -> Varchar,
        hab_is_favorite -> Bool,
        user_id -> Uuid,
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
    habit_recurrency (hab_rec_id) {
        hab_rec_id -> Uuid,
        #[max_length = 255]
        hab_rec_frequency_type -> Varchar,
        hab_rec_frequency_data -> Timestamp,
        hab_id -> Uuid,
    }
}

diesel::joinable!(habit_data_collected -> habit_recurrency (hab_rec_id));
diesel::joinable!(habit_recurrency -> habit (hab_id));

diesel::allow_tables_to_appear_in_same_query!(
    habit,
    habit_data_collected,
    habit_recurrency,
);
