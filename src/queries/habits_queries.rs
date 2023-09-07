use crate::{
    db::POSTGRES_POOL as pool, models::api::CategoryCreateSchema, models::database::Habit,
    schema::*,
};

use diesel::{prelude::*, result::Error};

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;

// TODO: Implement custom errors

// Get all of user habits
pub async fn get_all_user_habits(id: String) -> Result<Vec<Category>, Error> {
    habits::table
        .select(Habit::as_select())
        .filter(habit::usr_id.eq(id))
        .load::<Category>(&mut pool.get().unwrap())
}

// Add a habit
pub async fn add_habit(data: CategoryCreateSchema) -> Result<usize, Error> {
    let category = Category {
        cat_id: Uuid::new_v4(),
        cat_name: data.name,
    };

    diesel::insert_into(category::table)
        .values(&category)
        .execute(&mut pool.get().unwrap())
}

// Delete category
pub async fn delete_category(id: Uuid) -> Result<usize, Error> {
    diesel::delete(category::table.filter(category::cat_id.eq(id)))
        .execute(&mut pool.get().unwrap())
}

// Update a category
pub async fn update_category(id: Uuid, data: CategoryCreateSchema) -> Result<usize, Error> {
    diesel::update(category::table.filter(category::cat_id.eq(id)))
        .set((category::cat_name.eq(data.name),))
        .execute(&mut pool.get().unwrap())
}

// Filter a specific category
pub async fn get_category_by_id(id: Uuid) -> Result<Category, Error> {
    category::table
        .select(Category::as_select())
        .find(id)
        .first(&mut pool.get().unwrap())
}
