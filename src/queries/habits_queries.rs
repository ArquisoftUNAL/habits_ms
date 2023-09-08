use crate::{
    db::POSTGRES_POOL as pool,
    models::{api::habit_api_models::*, database::Habit},
    schema::*,
};

use diesel::{prelude::*, result::Error};

use uuid::Uuid;

// TODO: Implement custom errors

// Get all of user habits
pub async fn get_all_user_habits(id: &String) -> Result<Vec<Habit>, Error> {
    habit::table
        .select(Habit::as_select())
        .filter(habit::usr_id.eq(id))
        .load::<Habit>(&mut pool.get().unwrap())
}

// Add an habit
pub async fn add_habit(data: HabitCreateSchema) -> Result<usize, Error> {
    let habit = Habit {
        hab_id: Uuid::new_v4(),
        hab_name: data.name,
        hab_description: data.description,
        hab_created_at: chrono::Local::now().naive_local(),
        hab_updated_at: chrono::Local::now().naive_local(),
        hab_is_favorite: data.is_favourite,
        hab_type: data.kind,
        usr_id: data.user_id,
        cat_id: data.category,
    };

    diesel::insert_into(habit::table)
        .values(&habit)
        .execute(&mut pool.get().unwrap())
}

// Delete habit
pub async fn delete_habit(id: Uuid) -> Result<usize, Error> {
    diesel::delete(habit::table.filter(habit::hab_id.eq(id))).execute(&mut pool.get().unwrap())
}

// Update an habit
pub async fn update_habit(id: Uuid, data: HabitCreateSchema) -> Result<usize, Error> {
    diesel::update(habit::table.filter(habit::hab_id.eq(id)))
        .set((
            habit::hab_name.eq(data.name),
            habit::hab_description.eq(data.description),
            habit::hab_is_favorite.eq(data.is_favourite),
            habit::hab_type.eq(data.kind),
            habit::usr_id.eq(data.user_id),
            habit::hab_updated_at.eq(chrono::Local::now().naive_local()),
            habit::cat_id.eq(data.category),
        ))
        .execute(&mut pool.get().unwrap())
}

// Filter a specific habit
pub async fn get_habit_by_id(id: Uuid) -> Result<Habit, Error> {
    habit::table
        .select(Habit::as_select())
        .find(id)
        .first(&mut pool.get().unwrap())
}
