use crate::{
    db::DBManager,
    error::Error,
    models::api::category_api_models::*,
    models::database::Category,
    schema::*,
    utils::{DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT},
};

use diesel::prelude::*;

use uuid::Uuid;

impl DBManager {
    // Get all categories
    pub fn get_all_categories(
        &self,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<Vec<Category>, Error> {
        let page = page.unwrap_or(1);
        let mut per_page = per_page.unwrap_or(DEFAULT_QUERY_LIMIT);

        if per_page > MAX_QUERY_LIMIT {
            per_page = MAX_QUERY_LIMIT;
        }

        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let search = category::table
            .select(Category::as_select())
            .limit(per_page)
            .offset((page - 1) * per_page)
            .load(&mut conn.unwrap());

        if search.is_err() {
            return Err(Error::QueryError(search.err().unwrap()));
        }

        Ok(search.unwrap())
    }

    // Add a category
    pub fn add_category(&self, data: CategoryCreateSchema) -> Result<Uuid, Error> {
        let category = Category {
            cat_id: Uuid::new_v4(),
            cat_name: data.name,
        };

        let conn = self.get_write_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let result = diesel::insert_into(category::table)
            .values(&category)
            .execute(&mut conn.unwrap())
            .map(|_| category.cat_id);

        if result.is_err() {
            return Err(Error::QueryError(result.err().unwrap()));
        }

        Ok(result.unwrap())
    }

    // Delete category
    pub fn delete_category(&self, id: Uuid) -> Result<Uuid, Error> {
        let conn = self.get_write_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let result = diesel::delete(category::table.filter(category::cat_id.eq(id)))
            .execute(&mut conn.unwrap())
            .map(|_| id);

        if result.is_err() {
            return Err(Error::QueryError(result.err().unwrap()));
        }

        Ok(result.unwrap())
    }

    // Update a category
    pub fn update_category(&self, id: Uuid, data: CategoryUpdateSchema) -> Result<Uuid, Error> {
        let conn = self.get_write_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let result = diesel::update(category::table.filter(category::cat_id.eq(id)))
            .set(&data)
            .execute(&mut conn.unwrap())
            .map(|_| id);

        if result.is_err() {
            return Err(Error::QueryError(result.err().unwrap()));
        }

        Ok(result.unwrap())
    }

    // Filter a specific category
    pub fn get_category_by_id(&self, id: Uuid) -> Result<Category, Error> {
        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
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
