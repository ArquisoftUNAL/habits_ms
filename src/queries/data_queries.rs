use crate::{
    db::POSTGRES_POOL as pool,
    models::{
        api::data_api_models::*,
        database::{Habit, HabitDataCollected, HabitRecurrency},
    },
    schema::*,
};

use diesel::{prelude::*, result::Error};

use uuid::Uuid;

// TODO: Implement custom errors

// Get all of habit recurrences
pub async fn get_all_recurrency_data(id: Uuid) -> Result<Vec<HabitDataCollected>, Error> {
    habit_data_collected::table
        .select(HabitDataCollected::as_select())
        .filter(habit_data_collected::hab_rec_id.eq(id))
        .load::<HabitDataCollected>(&mut pool.get().unwrap())
}

// Add a habit data record
pub async fn add_habit_data(data: HabitDataSchema) -> Result<usize, Error> {
    let habit_data = HabitDataCollected {
        hab_dat_id: Uuid::new_v4(),
        hab_dat_amount: data.amount,
        hab_dat_collected_at: chrono::Local::now().naive_local(),
        hab_rec_id: data.recurrency_id,
    };

    diesel::insert_into(habit_data_collected::table)
        .values(&habit_data)
        .execute(&mut pool.get().unwrap())
}

// Delete recurrence
pub async fn delete_habit_data(id: Uuid) -> Result<usize, Error> {
    diesel::delete(habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(id)))
        .execute(&mut pool.get().unwrap())
}

// Update an habit
pub async fn update_habit_data(id: Uuid, data: HabitDataSchema) -> Result<usize, Error> {
    diesel::update(habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(id)))
        .set((
            habit_data_collected::hab_dat_amount.eq(data.amount),
            habit_data_collected::hab_dat_collected_at.eq(chrono::Local::now().naive_local()),
            habit_data_collected::hab_rec_id.eq(data.recurrency_id),
        ))
        .execute(&mut pool.get().unwrap())
}

// Filter a specific habit
pub async fn get_habit_data_by_id(id: Uuid) -> Result<HabitDataCollected, Error> {
    habit_data_collected::table
        .select(HabitDataCollected::as_select())
        .find(id)
        .first(&mut pool.get().unwrap())
}

// Join habit data with a set set of habits (including recurrences as well)
pub async fn join_habits_recurrences(
    habits: Vec<Habit>,
) -> Vec<(Habit, Vec<(HabitRecurrency, Vec<HabitDataCollected>)>)> {
    let recurrences = HabitRecurrency::belonging_to(&habits)
        .select(HabitRecurrency::as_select())
        .load::<HabitRecurrency>(&mut pool.get().unwrap())
        .unwrap();
    let habits_data = HabitDataCollected::belonging_to(&recurrences)
        .select(HabitDataCollected::as_select())
        .load::<HabitDataCollected>(&mut pool.get().unwrap())
        .unwrap();
    let grouped_habits_data = habits_data.grouped_by(&recurrences);
    let recurrences_and_data = recurrences
        .into_iter()
        .zip(grouped_habits_data)
        .grouped_by(&habits);
    habits.into_iter().zip(recurrences_and_data).collect()
}
