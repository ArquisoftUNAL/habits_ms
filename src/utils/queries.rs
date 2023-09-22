use crate::models::{
    api::habit_api_models::*,
    database::{Habit, HabitDataCollected},
};

pub fn join_habit_with_data(
    habit_item: Habit,
    data_array: Vec<HabitDataCollected>,
) -> HabitWithData {
    HabitWithData {
        hab_id: habit_item.hab_id,
        hab_name: habit_item.hab_name,
        hab_description: habit_item.hab_description,
        hab_is_favorite: habit_item.hab_is_favorite,
        hab_is_yn: habit_item.hab_is_yn,
        hab_color: habit_item.hab_color,
        hab_units: habit_item.hab_units,
        hab_goal: habit_item.hab_goal,
        hab_freq_type: habit_item.hab_freq_type,
        usr_id: habit_item.usr_id,
        cat_id: habit_item.cat_id,
        data: data_array,
    }
}
