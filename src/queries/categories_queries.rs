use crate::{
    db::DBManager, error::Error, models::api::category_api_models::*, models::database::Category,
    schema::*,
};

use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Get all categories
    pub fn get_all_categories(&self) -> Result<Vec<Category>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let search = category::table
            .select(Category::as_select())
            .load(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }
    // Add a category
    pub fn add_category(&self, data: CategoryCreateSchema) -> Result<usize, Error> {
        let category = Category {
            cat_id: Uuid::new_v4(),
            cat_name: data.name,
        };

        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let result = diesel::insert_into(category::table)
            .values(&category)
            .execute(&mut conn.unwrap());

        if result.is_err() {
            return Err(Error::QueryError(result.err().unwrap()));
        }

        Ok(result.unwrap())
    }

    // Delete category
    pub fn delete_category(&self, id: Uuid) -> Result<usize, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let result = diesel::delete(category::table.filter(category::cat_id.eq(id)))
            .execute(&mut conn.unwrap());

        if result.is_err() {
            return Err(Error::QueryError(result.err().unwrap()));
        }

        Ok(result.unwrap())
    }

    // Update a category
    pub fn update_category(&self, id: Uuid, data: CategoryCreateSchema) -> Result<usize, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let result = diesel::update(category::table.filter(category::cat_id.eq(id)))
            .set((category::cat_name.eq(data.name),))
            .execute(&mut conn.unwrap());

        if result.is_err() {
            return Err(Error::QueryError(result.err().unwrap()));
        }

        Ok(result.unwrap())
    }

    // Filter a specific category
    pub fn get_category_by_id(&self, id: Uuid) -> Result<Category, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let result = category::table
            .select(Category::as_select())
            .find(id)
            .first(&mut conn.unwrap());

        if result.is_err() {
            return Err(Error::QueryError(result.err().unwrap()));
        }

        Ok(result.unwrap())
    }
}
