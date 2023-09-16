use crate::models::{
    api::{habit_api_models::*, recurrence_api_models::*},
    database::{Habit, HabitDataCollected, HabitRecurrence},
};

pub fn join_habit_with_recurrences(
    habit_item: Habit,
    recurrences_array: Vec<HabitRecurrence>,
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

pub fn join_recurrence_with_data(
    recurrence_item: HabitRecurrence,
    data_array: Vec<HabitDataCollected>,
) -> RecurrenceWithData {
    RecurrenceWithData {
        hab_rec_id: recurrence_item.hab_rec_id,

        hab_id: recurrence_item.hab_id,

        hab_rec_freq_type: recurrence_item.hab_rec_freq_type,

        hab_rec_freq_data: recurrence_item.hab_rec_freq_data,

        hab_rec_goal: recurrence_item.hab_rec_goal,

        data: data_array,
    }
}

pub fn join_habit_recurrence_and_data(
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
