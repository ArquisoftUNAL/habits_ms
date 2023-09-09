use crate::{
    db::DBManager, error::Error, models::api::habit_api_models::*, models::database::Habit,
    schema::*,
};

use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Get all of user habits
    pub fn get_all_user_habits(&self, id: &String) -> Result<Vec<Habit>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = habit::table
            .select(Habit::as_select())
            .filter(habit::usr_id.eq(id))
            .load::<Habit>(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Add an habit
    pub fn add_habit(&self, data: HabitCreateSchema) -> Result<usize, Error> {
        let habit = Habit {
            hab_id: Uuid::new_v4(),
            hab_name: data.name,
            hab_description: data.description,
            hab_created_at: chrono::Local::now().naive_local(),
            hab_updated_at: chrono::Local::now().naive_local(),
            hab_is_favorite: data.is_favourite,
            hab_type: data.kind,
            hab_color: data.color,
            hab_units: data.units,
            usr_id: data.user_id,
            cat_id: data.category,
        };

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::insert_into(habit::table)
            .values(&habit)
            .execute(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Delete habit
    pub fn delete_habit(&self, id: Uuid) -> Result<usize, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search =
            diesel::delete(habit::table.filter(habit::hab_id.eq(id))).execute(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Update an habit
    pub fn update_habit(&self, id: Uuid, data: HabitCreateSchema) -> Result<usize, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = diesel::update(habit::table.filter(habit::hab_id.eq(id)))
            .set((
                habit::hab_name.eq(data.name),
                habit::hab_description.eq(data.description),
                habit::hab_is_favorite.eq(data.is_favourite),
                habit::hab_type.eq(data.kind),
                habit::hab_units.eq(data.units),
                habit::usr_id.eq(data.user_id),
                habit::hab_updated_at.eq(chrono::Local::now().naive_local()),
                habit::cat_id.eq(data.category),
            ))
            .execute(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Filter a specific habit
    pub fn get_habit_by_id(&self, id: Uuid) -> Result<Habit, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
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
}
