use crate::{
    db::DBManager,
    error::Error,
    models::api::data_api_models::*,
    models::{
        api::habit_api_models::HabitWithData,
        database::{Habit, HabitDataCollected},
    },
    schema::*,
    utils::queries::join_habit_with_data,
    utils::time::{MAXIMUM_DATE, MINIMUM_DATE},
    utils::{DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT},
};

use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Check if habit is accessible by user
    pub fn is_habitdata_accessible_by_user(
        &self,
        user_id: String,
        habitdata_id: Uuid,
    ) -> Result<bool, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        // Check if habit exists
        let search = habit::table
            .inner_join(habit_data_collected::table)
            .select(habit::usr_id)
            .filter(habit_data_collected::hab_dat_id.eq(habitdata_id))
            .first::<String>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        // Check if user is the owner of the habit
        Ok(search.unwrap() == user_id)
    }

    // Get all of habit recurrences
    pub fn get_all_habit_data(
        &self,
        id: Uuid,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<HabitDataCollected>, Error> {
        let page = page.unwrap_or(1);
        let mut per_page = per_page.unwrap_or(DEFAULT_QUERY_LIMIT);

        if per_page > MAX_QUERY_LIMIT {
            per_page = MAX_QUERY_LIMIT;
        }

        let start_date = start_date.unwrap_or(MINIMUM_DATE.unwrap());
        let end_date: chrono::NaiveDate = end_date.unwrap_or(MAXIMUM_DATE.unwrap());

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit_data_collected::table
            .select(HabitDataCollected::as_select())
            .filter(habit_data_collected::hab_id.eq(id))
            .filter(habit_data_collected::hab_dat_collected_at.ge(start_date))
            .filter(habit_data_collected::hab_dat_collected_at.le(end_date))
            .limit(per_page)
            .offset((page - 1) * per_page)
            .order_by(habit_data_collected::hab_dat_collected_at.asc())
            .load::<HabitDataCollected>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Add a habit data record
    pub fn add_habit_data(&self, data: HabitDataCreateSchema) -> Result<HabitDataCollected, Error> {
        println!("Log 2.5: {:?}", chrono::Utc::now().naive_utc().date());

        let habit_data = HabitDataCollected {
            hab_dat_id: Uuid::new_v4(),
            hab_dat_amount: data.amount,
            hab_dat_collected_at: data
                .collected_at
                .unwrap_or_else(|| chrono::Utc::now().naive_utc().date()),
            hab_id: data.habit_id,
        };

        println!("Log 3: {:?}", habit_data);

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let query = diesel::insert_into(habit_data_collected::table)
            .values(&habit_data)
            .get_result(&mut conn.unwrap());

        if query.is_err() {
            return Err(Error::QueryError(query.err().unwrap()));
        }

        Ok(query.unwrap())
    }

    // Delete recurrence
    pub fn delete_habit_data(&self, id: Uuid) -> Result<HabitDataCollected, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let query = diesel::delete(
            habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(id)),
        )
        .get_result::<HabitDataCollected>(&mut conn.unwrap());

        if query.is_err() {
            return Err(Error::QueryError(query.err().unwrap()));
        }

        Ok(query.unwrap())
    }

    // Update an habit
    pub fn update_habit_data(
        &self,
        id: Uuid,
        data: HabitDataUpdateSchema,
    ) -> Result<HabitDataCollected, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let query = diesel::update(
            habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(id)),
        )
        .set(&data)
        .get_result::<HabitDataCollected>(&mut conn.unwrap());

        if query.is_err() {
            return Err(Error::QueryError(query.err().unwrap()));
        }

        Ok(query.unwrap())
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

    // Get all user data
    pub fn get_all_user_habitdata(
        &self,
        user_id: String,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<HabitDataCollected>, Error> {
        let page = page.unwrap_or(1);
        let mut per_page = per_page.unwrap_or(DEFAULT_QUERY_LIMIT);

        if per_page > MAX_QUERY_LIMIT {
            per_page = MAX_QUERY_LIMIT;
        }

        let start_date = start_date.unwrap_or(MINIMUM_DATE.unwrap());
        let end_date: chrono::NaiveDate = end_date.unwrap_or(MAXIMUM_DATE.unwrap());

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        // Get all habits from user
        let habits_data = habit_data_collected::table
            .inner_join(habit::table)
            .select(HabitDataCollected::as_select())
            .filter(habit::usr_id.eq(user_id))
            .filter(habit_data_collected::hab_dat_collected_at.ge(start_date))
            .filter(habit_data_collected::hab_dat_collected_at.le(end_date))
            .limit(per_page)
            .offset((page - 1) * per_page)
            .order_by(habit_data_collected::hab_dat_collected_at.asc())
            .load::<HabitDataCollected>(&mut conn.unwrap());

        if habits_data.is_err() {
            return Err(Error::QueryError(habits_data.err().unwrap()));
        }

        let habits_data = habits_data.unwrap();

        Ok(habits_data)
    }

    // Join habit data with a set set of habits (including recurrences as well)
    pub fn join_habits_data(
        &self,
        habits: Vec<Habit>,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
    ) -> Result<Vec<HabitWithData>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let mut conn = conn.unwrap();

        let habits_data = HabitDataCollected::belonging_to(&habits)
            .select(HabitDataCollected::as_select())
            .filter(
                habit_data_collected::hab_dat_collected_at
                    .ge(start_date.unwrap_or(MINIMUM_DATE.unwrap())),
            )
            .filter(
                habit_data_collected::hab_dat_collected_at
                    .le(end_date.unwrap_or(MAXIMUM_DATE.unwrap())),
            )
            .order_by(habit_data_collected::hab_dat_collected_at.desc())
            .load::<HabitDataCollected>(&mut conn);

        if habits_data.is_err() {
            return Err(Error::QueryError(habits_data.err().unwrap()));
        }

        let habits_data = habits_data.unwrap();

        let grouped_habits_data = habits_data.grouped_by(&habits);
        let result = habits
            .into_iter()
            .zip(grouped_habits_data)
            .map(|(habit_item, habit_data)| join_habit_with_data(habit_item, habit_data))
            .collect();

        Ok(result)
    }
}
