use crate::{
    db::POSTGRES_POOL as pool, models::api::category_api_models::*, models::database::Category,
    schema::*,
};

use diesel::{prelude::*, result::Error};

use uuid::Uuid;

// TODO: Implement custom errors
// Get all categories
pub async fn get_all_categories() -> Result<Vec<Category>, Error> {
    category::table
        .select(Category::as_select())
        .load::<Category>(&mut pool.get().unwrap())
}

// Add a category
pub async fn add_category(data: CategoryCreateSchema) -> Result<usize, Error> {
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
