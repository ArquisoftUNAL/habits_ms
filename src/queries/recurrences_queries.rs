use crate::{
    db::DBManager,
    error::Error,
    models::api::{habit_api_models::*, recurrence_api_models::*},
    models::database::{Habit, HabitRecurrence},
    schema::*,
    utils::join_habit_with_recurrences,
};

use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Get all of habit recurrences
    pub fn get_all_habit_recurrences(&self, id: Uuid) -> Result<Vec<HabitRecurrence>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit_recurrence::table
            .select(HabitRecurrence::as_select())
            .filter(habit_recurrence::hab_id.eq(id))
            .load::<HabitRecurrence>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Add a recurrence
    pub fn add_recurrence(&self, data: RecurrenceCreateSchema) -> Result<Uuid, Error> {
        let recurrence = HabitRecurrence {
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

        let search = diesel::insert_into(habit_recurrence::table)
            .values(&recurrence)
            .execute(&mut conn.unwrap())
            .map(|_| recurrence.hab_rec_id);

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
            diesel::delete(habit_recurrence::table.filter(habit_recurrence::hab_rec_id.eq(id)))
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
            diesel::update(habit_recurrence::table.filter(habit_recurrence::hab_rec_id.eq(id)))
                .set(&data)
                .execute(&mut conn.unwrap())
                .map(|_| id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Filter a specific habit
    pub fn get_recurrence_by_id(&self, id: Uuid) -> Result<HabitRecurrence, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit_recurrence::table
            .select(HabitRecurrence::as_select())
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

        let recurrences = HabitRecurrence::belonging_to(&habits)
            .select(HabitRecurrence::as_select())
            .load::<HabitRecurrence>(&mut conn.unwrap());

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
