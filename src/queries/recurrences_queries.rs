use crate::{
    db::POSTGRES_POOL as pool,
    models::{
        api::recurrency_api_models::*,
        database::{Habit, HabitRecurrency},
    },
    schema::*,
};

use diesel::{prelude::*, result::Error, BelongingToDsl};

use uuid::Uuid;

// TODO: Implement custom errors

// Get all of habit recurrences
pub async fn get_all_habit_recurrences(id: Uuid) -> Result<Vec<HabitRecurrency>, Error> {
    habit_recurrency::table
        .select(HabitRecurrency::as_select())
        .filter(habit_recurrency::hab_id.eq(id))
        .load::<HabitRecurrency>(&mut pool.get().unwrap())
}

// Add a recurrence
pub async fn add_recurrence(data: RecurrencyCreateSchema) -> Result<usize, Error> {
    let recurrency = HabitRecurrency {
        hab_rec_id: Uuid::new_v4(),
        hab_rec_freq_data: data.frequency_data,
        hab_rec_freq_type: data.frequency_type,
        hab_id: data.habit_id,
    };

    diesel::insert_into(habit_recurrency::table)
        .values(&recurrency)
        .execute(&mut pool.get().unwrap())
}

// Delete recurrence
pub async fn delete_recurrence(id: Uuid) -> Result<usize, Error> {
    diesel::delete(habit_recurrency::table.filter(habit_recurrency::hab_rec_id.eq(id)))
        .execute(&mut pool.get().unwrap())
}

// Update an habit
pub async fn update_recurrence(id: Uuid, data: RecurrencyCreateSchema) -> Result<usize, Error> {
    diesel::update(habit_recurrency::table.filter(habit_recurrency::hab_rec_id.eq(id)))
        .set((
            habit_recurrency::hab_id.eq(data.habit_id),
            habit_recurrency::hab_rec_freq_data.eq(data.frequency_data),
            habit_recurrency::hab_rec_freq_type.eq(data.frequency_type),
            habit_recurrency::hab_id.eq(data.habit_id),
        ))
        .execute(&mut pool.get().unwrap())
}

// Filter a specific habit
pub async fn get_recurrence_by_id(id: Uuid) -> Result<HabitRecurrency, Error> {
    habit_recurrency::table
        .select(HabitRecurrency::as_select())
        .find(id)
        .first(&mut pool.get().unwrap())
}

// Join recurrences with a set set of habits
pub async fn join_habits_recurrences(habits: Vec<Habit>) -> Vec<(Habit, Vec<HabitRecurrency>)> {
    let recurrences = HabitRecurrency::belonging_to(&habits)
        .select(HabitRecurrency::as_select())
        .load::<HabitRecurrency>(&mut pool.get().unwrap())
        .unwrap()
        .grouped_by(&habits);

    habits.into_iter().zip(recurrences).collect()
}
