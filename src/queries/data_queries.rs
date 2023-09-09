use crate::{
    db::DBManager,
    error::Error,
    models::api::data_api_models::*,
    models::{
        api::habit_api_models::HabitRecurrencesAndData,
        database::{Habit, HabitDataCollected, HabitRecurrency},
    },
    schema::*,
    utils::{join_habit_recurrency_and_data, join_recurrency_with_data},
};

use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Get all of habit recurrences
    pub fn get_all_recurrency_data(&self, id: Uuid) -> Result<Vec<HabitDataCollected>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit_data_collected::table
            .select(HabitDataCollected::as_select())
            .filter(habit_data_collected::hab_rec_id.eq(id))
            .load::<HabitDataCollected>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Add a habit data record
    pub fn add_habit_data(&self, data: HabitDataSchema) -> Result<usize, Error> {
        let habit_data = HabitDataCollected {
            hab_dat_id: Uuid::new_v4(),
            hab_dat_amount: data.amount,
            hab_dat_collected_at: chrono::Local::now().naive_local(),
            hab_rec_id: data.recurrency_id,
        };

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::insert_into(habit_data_collected::table)
            .values(&habit_data)
            .execute(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Delete recurrence
    pub fn delete_habit_data(&self, id: Uuid) -> Result<usize, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::delete(
            habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(id)),
        )
        .execute(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Update an habit
    pub fn update_habit_data(&self, id: Uuid, data: HabitDataSchema) -> Result<usize, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::update(
            habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(id)),
        )
        .set((
            habit_data_collected::hab_dat_amount.eq(data.amount),
            habit_data_collected::hab_dat_collected_at.eq(chrono::Local::now().naive_local()),
            habit_data_collected::hab_rec_id.eq(data.recurrency_id),
        ))
        .execute(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Filter a specific habit
    pub fn get_habit_data_by_id(&self, id: Uuid) -> Result<HabitDataCollected, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit_data_collected::table
            .select(HabitDataCollected::as_select())
            .find(id)
            .first(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Join habit data with a set set of habits (including recurrences as well)
    pub fn join_habits_recurrences_data(
        &self,
        habits: Vec<Habit>,
    ) -> Result<Vec<HabitRecurrencesAndData>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let mut conn = conn.unwrap();

        let recurrences = HabitRecurrency::belonging_to(&habits)
            .select(HabitRecurrency::as_select())
            .load::<HabitRecurrency>(&mut conn);

        if recurrences.is_err() {
            return Err(Error::QueryError(recurrences.err().unwrap()));
        }

        let recurrences = recurrences.unwrap();

        let habits_data = HabitDataCollected::belonging_to(&recurrences)
            .select(HabitDataCollected::as_select())
            .load::<HabitDataCollected>(&mut conn);

        if habits_data.is_err() {
            return Err(Error::QueryError(habits_data.err().unwrap()));
        }

        let habits_data = habits_data.unwrap();

        let grouped_habits_data = habits_data.grouped_by(&recurrences);
        let recurrences_and_data = recurrences
            .into_iter()
            .zip(grouped_habits_data)
            .grouped_by(&habits);
        let result = habits
            .into_iter()
            .zip(recurrences_and_data)
            .map(|(habit_item, recurrence_with_data)| {
                let recurrence_with_data = recurrence_with_data
                    .into_iter()
                    .map(|(recurrence_item, data_array)| {
                        join_recurrency_with_data(recurrence_item, data_array)
                    })
                    .collect();

                join_habit_recurrency_and_data(habit_item, recurrence_with_data)
            })
            .collect();

        Ok(result)
    }
}
