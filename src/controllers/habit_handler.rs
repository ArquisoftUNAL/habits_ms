use crate::{
    db::POSTGRES_POOL as pool,
    models::api::{habit_api_models::*, *},
    models::database::Habit,
    schema::*,
};
use diesel::prelude::*;

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;

// CREATE Route
pub async fn create_habit_handler(body: HabitCreateRequest) -> Result<impl Reply, Rejection> {
    // Create model from request body
    let habit = Habit {
        hab_id: uuid::Uuid::new_v4(),
        hab_name: body.name,
        hab_description: body.description,
        hab_created_at: chrono::Local::now().naive_local(),
        hab_updated_at: chrono::Local::now().naive_local(),
        hab_is_favorite: body.is_favourite,
        hab_type: body.kind,
        usr_id: body.user_id,
        cat_id: body.category,
    };
    // Add habit to database
    let result = diesel::insert_into(habit::table)
        .values(&habit)
        .execute(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error creating habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = HabitCreateResponse {
        status: 201,
        message: "Habit created successfully".to_string(),
        hab_id: habit.hab_id,
    };
    Ok(json(&response))
}

// READ Route
pub async fn get_habits_handler(user_id: String) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let result = habit::table
        .select(Habit::as_select())
        .filter(habit::usr_id.eq(user_id))
        .load::<Habit>(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error getting habits: {}", error),
        };
        return Ok(json(&response));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    if result.len() == 0 {
        let response = GeneralResponse {
            status: 404,
            message: "User / Habits not found".to_string(),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = HabitMultipleQueryResponse {
        status: 200,
        habits: result,
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_habits_handler(habit_id: Uuid) -> Result<impl Reply, Rejection> {
    // Delete habit from database
    let result = diesel::delete(habit::table.filter(habit::hab_id.eq(habit_id)))
        .execute(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error deleting habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        status: 200,
        message: "Habit deleted successfully".to_string(),
    };
    Ok(json(&response))
}

// UPDATE (PATCH) Route
pub async fn update_habits_handler(
    habit_id: Uuid,
    body: HabitCreateRequest,
) -> Result<impl Reply, Rejection> {
    let result = diesel::update(habit::table.filter(habit::hab_id.eq(habit_id)))
        .set((
            habit::hab_name.eq(body.name),
            habit::hab_description.eq(body.description),
            habit::hab_is_favorite.eq(body.is_favourite),
            habit::hab_type.eq(body.kind),
            habit::usr_id.eq(body.user_id),
            habit::hab_updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .execute(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error updating habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        status: 200,
        message: "Habit updated successfully".to_string(),
    };

    Ok(json(&response))
}
