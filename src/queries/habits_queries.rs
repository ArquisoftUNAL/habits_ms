use crate::{
    db::DBManager,
    error::Error,
    models::api::habit_api_models::*,
    models::database::Habit,
    schema::*,
    utils::{
        time::{get_next_closure_date, DateRange},
        DEFAULT_QUERY_LIMIT, HABIT_CREATION_DATE_AS_REFERENCE, MAX_QUERY_LIMIT,
    },
};

use chrono::NaiveDate;
use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Check if habit is accessible by user
    pub fn is_habit_accessible_by_user(
        &self,
        user_id: String,
        habit_id: Uuid,
    ) -> Result<bool, Error> {
        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        // Check if habit exists
        let search = habit::table
            .select(habit::usr_id)
            .filter(habit::hab_id.eq(habit_id))
            .first::<String>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        // Check if user is the owner of the habit
        Ok(search.unwrap() == user_id)
    }

    // Add an habit
    pub fn add_habit(&self, user_id: String, data: HabitCreateSchema) -> Result<Uuid, Error> {
        let current_datetime = chrono::Local::now().naive_local();
        let current_date = current_datetime.date();

        let closure_date: NaiveDate;

        if HABIT_CREATION_DATE_AS_REFERENCE {
            closure_date = current_date;
        } else {
            closure_date = DateRange::get_next_closest_date(
                data.frequency_type,
                // Change if habit start should be another (usually a week later from current date)
                Some(current_date),
                // Change to None when reference date should be another (usually a constant)
                None,
            );
        }

        let habit = Habit {
            hab_id: Uuid::new_v4(),
            hab_name: data.name,
            hab_description: data.description,
            hab_created_at: current_datetime,
            hab_updated_at: current_datetime,
            hab_is_favorite: data.is_favorite,
            hab_is_yn: data.is_yn,
            hab_color: data.color,
            hab_units: data.units,
            hab_goal: data.goal,
            hab_freq_type: data.frequency_type,

            hab_next_closure_date: closure_date,
            hab_location: data.location,

            usr_id: user_id,
            cat_id: data.category,
        };

        let conn = self.get_write_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let search = diesel::insert_into(habit::table)
            .values(&habit)
            .execute(&mut conn.unwrap())
            .map(|_| habit.hab_id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Delete habit
    pub fn delete_habit(&self, id: Uuid) -> Result<Uuid, Error> {
        let conn = self.get_write_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let search = diesel::delete(habit::table.filter(habit::hab_id.eq(id)))
            .execute(&mut conn.unwrap())
            .map(|_| id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Update an habit
    pub fn update_habit(&self, id: Uuid, data: HabitUpdateSchema) -> Result<Uuid, Error> {
        let conn = self.get_write_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let search = diesel::update(habit::table.filter(habit::hab_id.eq(id)))
            .set(&data)
            .execute(&mut conn.unwrap())
            .map(|_| id);

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Get all of user habits
    pub fn get_all_user_habits(
        &self,
        id: String,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<Habit>, Error> {
        let page = page.unwrap_or(1);
        let mut per_page = per_page.unwrap_or(DEFAULT_QUERY_LIMIT);

        if per_page > MAX_QUERY_LIMIT {
            per_page = MAX_QUERY_LIMIT;
        }

        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let search = habit::table
            .select(Habit::as_select())
            .filter(habit::usr_id.eq(id))
            .limit(per_page.into())
            .offset((page - 1) * per_page)
            .load::<Habit>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Get all category habits
    pub fn get_all_category_habits(
        &self,
        id: Uuid,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<Habit>, Error> {
        let page = page.unwrap_or(1);
        let mut per_page = per_page.unwrap_or(DEFAULT_QUERY_LIMIT);

        if per_page > MAX_QUERY_LIMIT {
            per_page = MAX_QUERY_LIMIT;
        }

        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let search = habit::table
            .select(Habit::as_select())
            .filter(habit::cat_id.eq(id))
            .limit(per_page.into())
            .offset((page - 1) * per_page)
            .load::<Habit>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Get all category habits
    pub fn get_all_user_category_habits(
        &self,
        user_id: String,
        cat_id: Uuid,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<Habit>, Error> {
        let page = page.unwrap_or(1);
        let mut per_page = per_page.unwrap_or(DEFAULT_QUERY_LIMIT);

        if per_page > MAX_QUERY_LIMIT {
            per_page = MAX_QUERY_LIMIT;
        }

        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let search = habit::table
            .select(Habit::as_select())
            .filter(habit::cat_id.eq(cat_id))
            .filter(habit::usr_id.eq(user_id))
            .limit(per_page.into())
            .offset((page - 1) * per_page)
            .load::<Habit>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Filter a specific habit
    pub fn get_habit_by_id(&self, id: Uuid) -> Result<Habit, Error> {
        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let search = habit::table
            .select(Habit::as_select())
            .find(id)
            .first(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Get pending habits and update their closure date
    pub fn get_update_pending_habits(&self) -> Result<Vec<Habit>, Error> {
        let current_datetime = chrono::Local::now().naive_local();
        let current_date = current_datetime.date();

        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        // Get habits to update
        let search = habit::table
            .select(Habit::as_select())
            .filter(habit::hab_next_closure_date.le(current_date))
            .load::<Habit>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        let conn = self.get_write_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        // Update all habits that are pending
        let update = diesel::update(habit::table)
            .set(habit::hab_next_closure_date.eq(get_next_closure_date(
                habit::hab_freq_type,
                habit::hab_next_closure_date,
            )))
            .filter(habit::hab_next_closure_date.le(current_date))
            .get_result::<Habit>(&mut conn.unwrap());

        if update.is_err() {
            return Err(Error::QueryError(update.err().unwrap()));
        }

        Ok(search.unwrap())
    }
}
