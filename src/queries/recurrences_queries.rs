use crate::{
    db::DBManager,
    error::Error,
    models::api::{habit_api_models::*, recurrency_api_models::*},
    models::database::{Habit, HabitRecurrency},
    schema::*,
    utils::join_habit_with_recurrences,
};

use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Get all of habit recurrences
    pub fn get_all_habit_recurrences(&self, id: Uuid) -> Result<Vec<HabitRecurrency>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit_recurrency::table
            .select(HabitRecurrency::as_select())
            .filter(habit_recurrency::hab_id.eq(id))
            .load::<HabitRecurrency>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Add a recurrence
    pub fn add_recurrence(&self, data: RecurrencyCreateSchema) -> Result<Uuid, Error> {
        let recurrency = HabitRecurrency {
            hab_rec_id: Uuid::new_v4(),
            hab_rec_freq_data: data.frequency_data,
            hab_rec_freq_type: data.frequency_type,
            hab_id: data.habit_id,
            hab_rec_goal: data.goal,
        };

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::insert_into(habit_recurrency::table)
            .values(&recurrency)
            .execute(&mut conn.unwrap())
            .map(|_| recurrency.hab_rec_id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Delete recurrence
    pub fn delete_recurrence(&self, id: Uuid) -> Result<Uuid, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search =
            diesel::delete(habit_recurrency::table.filter(habit_recurrency::hab_rec_id.eq(id)))
                .execute(&mut conn.unwrap())
                .map(|_| id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Update an habit
    pub fn update_recurrence(&self, id: Uuid, data: RecurrenceUpdateSchema) -> Result<Uuid, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search =
            diesel::update(habit_recurrency::table.filter(habit_recurrency::hab_rec_id.eq(id)))
                .set(&data)
                .execute(&mut conn.unwrap())
                .map(|_| id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Filter a specific habit
    pub fn get_recurrence_by_id(&self, id: Uuid) -> Result<HabitRecurrency, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit_recurrency::table
            .select(HabitRecurrency::as_select())
            .find(id)
            .first(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Join recurrences with a set set of habits
    pub fn join_habits_recurrences(
        &self,
        habits: Vec<Habit>,
    ) -> Result<Vec<HabitWithRecurrences>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let recurrences = HabitRecurrency::belonging_to(&habits)
            .select(HabitRecurrency::as_select())
            .load::<HabitRecurrency>(&mut conn.unwrap());

        if recurrences.is_err() {
            return Err(Error::QueryError(recurrences.err().unwrap()));
        }

        let recurrences = recurrences.unwrap();
        let result = recurrences.grouped_by(&habits);

        let result = habits
            .into_iter()
            .zip(result)
            .map(|(habit_item, recurrences_array)| {
                join_habit_with_recurrences(habit_item, recurrences_array)
            })
            .collect();

        Ok(result)
    }
}
