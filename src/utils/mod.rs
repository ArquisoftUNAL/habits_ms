use crate::{
    db::{DBManager, PostgresPool},
    models::{
        api::{habit_api_models::*, recurrency_api_models::*},
        database::{Habit, HabitDataCollected, HabitRecurrency},
    },
};
use warp::{Filter, Rejection};

pub fn with_db_manager(
    pool: PostgresPool,
) -> impl Filter<Extract = (DBManager,), Error = Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(
            |pool: PostgresPool| async move { Ok::<DBManager, Rejection>(DBManager::new(pool)) },
        )
}

pub fn join_habit_with_recurrences(
    habit_item: Habit,
    recurrences_array: Vec<HabitRecurrency>,
) -> HabitWithRecurrences {
    HabitWithRecurrences {
        hab_id: habit_item.hab_id,
        hab_name: habit_item.hab_name,
        hab_description: habit_item.hab_description,
        hab_is_favorite: habit_item.hab_is_favorite,
        hab_is_yn: habit_item.hab_is_yn,
        hab_color: habit_item.hab_color,
        hab_units: habit_item.hab_units,
        usr_id: habit_item.usr_id,
        cat_id: habit_item.cat_id,
        recurrences: recurrences_array,
    }
}

pub fn join_recurrency_with_data(
    recurrency_item: HabitRecurrency,
    data_array: Vec<HabitDataCollected>,
) -> RecurrenceWithData {
    RecurrenceWithData {
        hab_rec_id: recurrency_item.hab_rec_id,

        hab_id: recurrency_item.hab_id,

        hab_rec_freq_type: recurrency_item.hab_rec_freq_type,

        hab_rec_freq_data: recurrency_item.hab_rec_freq_data,

        data: data_array,
    }
}

pub fn join_habit_recurrency_and_data(
    habit_item: Habit,
    recurrences_array: Vec<RecurrenceWithData>,
) -> HabitRecurrencesAndData {
    HabitRecurrencesAndData {
        hab_id: habit_item.hab_id,
        hab_name: habit_item.hab_name,
        hab_description: habit_item.hab_description,
        hab_is_favorite: habit_item.hab_is_favorite,
        hab_is_yn: habit_item.hab_is_yn,
        hab_color: habit_item.hab_color,
        hab_units: habit_item.hab_units,
        usr_id: habit_item.usr_id,
        cat_id: habit_item.cat_id,

        recurrences: recurrences_array,
    }
}
