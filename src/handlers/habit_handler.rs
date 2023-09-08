use crate::{
    models::api::{habit_api_models::*, *},
    queries::habits_queries,
};

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;

// POST Route
pub async fn create_habit_handler(data: HabitCreateSchema) -> Result<impl Reply, Rejection> {
    // Create model from request body
    let result = habits_queries::add_habit(data).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error creating habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        message: format!("Habit created successfully"),
    };

    Ok(json(&response))
}

// GET Route
pub async fn get_habits_handler(id: String) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let user_id = id.clone();

    let result = habits_queries::get_all_user_habits(&user_id).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!(
                "Error getting habits: {} for user with ID: {}",
                error, &user_id
            ),
        };
        return Ok(json(&response));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    if result.len() == 0 {
        let response = GeneralResponse {
            message: "User / Habits not found".to_string(),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = HabitMultipleQueryResponse {
        message: format!(
            "Successfully retrieved habits for user with ID: {}",
            &user_id
        ),
        habits: result,
    };

    Ok(json(&response))
}

// GET Route
pub async fn get_habit_by_id_handler(id: Uuid) -> Result<impl Reply, Rejection> {
    // Get habits from database

    let result = habits_queries::get_habit_by_id(id).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error getting habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    // Return response
    let response = HabitSingleQueryResponse {
        message: format!("Successfully retrieved habit"),
        habit: result,
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_habits_handler(id: Uuid) -> Result<impl Reply, Rejection> {
    // Delete habit from database
    let result = habits_queries::delete_habit(id).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error deleting habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        message: "Habit deleted successfully".to_string(),
    };
    Ok(json(&response))
}

// UPDATE (PATCH) Route
pub async fn update_habits_handler(
    id: Uuid,
    data: HabitCreateSchema,
) -> Result<impl Reply, Rejection> {
    let result = habits_queries::update_habit(id, data).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error updating habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        message: "Habit updated successfully".to_string(),
    };

    Ok(json(&response))
}
