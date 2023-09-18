use crate::{
    db::DBManager,
    error::Error,
    models::api::data_api_models::*,
    models::{
        api::{
            habit_api_models::HabitRecurrencesAndData, recurrence_api_models::RecurrenceWithData,
        },
        database::{Habit, HabitDataCollected, HabitRecurrence},
    },
    schema::*,
    utils::queries::{join_habit_recurrence_and_data, join_recurrence_with_data},
    utils::{DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT},
};

use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Get all of habit recurrences
    pub fn get_all_recurrence_data(
        &self,
        id: Uuid,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<HabitDataCollected>, Error> {
        let page = page.unwrap_or(1);
        let mut per_page = per_page.unwrap_or(DEFAULT_QUERY_LIMIT);

        if per_page > MAX_QUERY_LIMIT {
            per_page = MAX_QUERY_LIMIT;
        }

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit_data_collected::table
            .select(HabitDataCollected::as_select())
            .filter(habit_data_collected::hab_rec_id.eq(id))
            .limit(per_page)
            .offset((page - 1) * per_page)
            .order_by(habit_data_collected::hab_dat_collected_at.asc())
            .load::<HabitDataCollected>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Get all habit's recurrences with data
    pub fn get_all_habit_recurrences_data(
        &self,
        id: Uuid,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<RecurrenceWithData>, Error> {
        let page = page.unwrap_or(1);
        let mut per_page = per_page.unwrap_or(DEFAULT_QUERY_LIMIT);

        if per_page > MAX_QUERY_LIMIT {
            per_page = MAX_QUERY_LIMIT;
        }

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let mut conn = conn.unwrap();

        // Get recurrences from database
        let recurrences = habit_recurrence::table
            .select(HabitRecurrence::as_select())
            .filter(habit_recurrence::hab_id.eq(id))
            .limit(per_page)
            .offset((page - 1) * per_page)
            .order_by(habit_recurrence::hab_rec_freq_data.asc())
            .load::<HabitRecurrence>(&mut conn);

        if recurrences.is_err() {
            return Err(Error::QueryError(recurrences.err().unwrap()));
        }

        let recurrences = recurrences.unwrap();

        // Get data from database
        let habits_data = HabitDataCollected::belonging_to(&recurrences)
            .select(HabitDataCollected::as_select())
            .order_by(habit_data_collected::hab_dat_collected_at.desc())
            .load::<HabitDataCollected>(&mut conn);

        if habits_data.is_err() {
            return Err(Error::QueryError(habits_data.err().unwrap()));
        }

        let habits_data = habits_data.unwrap();

        // Group data by recurrence
        let grouped_habits_data = habits_data.grouped_by(&recurrences);

        // Join recurrences with data
        let result = recurrences
            .into_iter()
            .zip(grouped_habits_data)
            .map(|(recurrence_item, data_array)| {
                join_recurrence_with_data(recurrence_item, data_array)
            })
            .collect();

        Ok(result)
    }

    // Add a habit data record
    pub fn add_habit_data(&self, data: HabitDataSchema) -> Result<Uuid, Error> {
        let habit_data = HabitDataCollected {
            hab_dat_id: Uuid::new_v4(),
            hab_dat_amount: data.amount,
            hab_dat_collected_at: data
                .collected_at
                .unwrap_or_else(|| chrono::Utc::now().naive_utc().date()),
            hab_rec_id: data.recurrence_id,
        };

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::insert_into(habit_data_collected::table)
            .values(&habit_data)
            .execute(&mut conn.unwrap())
            .map(|_| habit_data.hab_dat_id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Delete recurrence
    pub fn delete_habit_data(&self, id: Uuid) -> Result<Uuid, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::delete(
            habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(id)),
        )
        .execute(&mut conn.unwrap())
        .map(|_| id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Update an habit
    pub fn update_habit_data(&self, id: Uuid, data: HabitDataUpdateSchema) -> Result<Uuid, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::update(
            habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(id)),
        )
        .set(&data)
        .execute(&mut conn.unwrap())
        .map(|_| id);

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

        let recurrences = HabitRecurrence::belonging_to(&habits)
            .select(HabitRecurrence::as_select())
            .load::<HabitRecurrence>(&mut conn);

        if recurrences.is_err() {
            return Err(Error::QueryError(recurrences.err().unwrap()));
        }

        let recurrences = recurrences.unwrap();

        let habits_data = HabitDataCollected::belonging_to(&recurrences)
            .select(HabitDataCollected::as_select())
            .order_by(habit_data_collected::hab_dat_collected_at.desc())
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
                        join_recurrence_with_data(recurrence_item, data_array)
                    })
                    .collect();

                join_habit_recurrence_and_data(habit_item, recurrence_with_data)
            })
            .collect();

        Ok(result)
    }
}
