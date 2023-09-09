use crate::{
    db::DBManager,
    models::api::{habit_api_models::*, *},
};

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;
use validator::Validate;

// POST Route
pub async fn create_habit_handler(
    manager: DBManager,
    data: HabitCreateSchema,
) -> Result<impl Reply, Rejection> {
    // Validate input
    let validation_result = HabitCreateSchema::validate(&data);
    if validation_result.is_err() {
        let error = validation_result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error validating habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Create model from request body
    let result = manager.add_habit(data);

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
pub async fn get_habits_handler(manager: DBManager, id: String) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let user_id = id.clone();

    let result = manager.get_all_user_habits(&user_id);

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

    println!("{:?}", response);

    Ok(json(&response))
}

// GET Route
pub async fn get_habit_by_id_handler(
    manager: DBManager,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    // Get habits from database

    let result = manager.get_habit_by_id(id);

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

// GET Route
pub async fn get_habits_recurrences_by_user_id(
    manager: DBManager,
    id: String,
) -> Result<impl Reply, Rejection> {
    let user_id = id.clone();
    let result = manager.get_all_user_habits(&user_id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!(
                "Error getting habits & recurrences: {} for user with ID: {}",
                error, &user_id
            ),
        };
        return Ok(json(&response));
    }

    let result = manager.join_habits_recurrences(result.unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!(
                "Error getting habits & recurrences: {} for user with ID: {}",
                error, &user_id
            ),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = HabitAndRecurrencesMultipleQueryResponse {
        message: format!(
            "Successfully retrieved habits & recurrences for user with ID: {}",
            &user_id
        ),
        habits: result.unwrap(),
    };

    Ok(json(&response))
}

// GET Route
pub async fn get_habits_recurrences_data_by_user_id(
    manager: DBManager,
    id: String,
) -> Result<impl Reply, Rejection> {
    let user_id = id.clone();
    let result = manager.get_all_user_habits(&user_id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!(
                "Error getting habits, recurrences & data: {} for user with ID: {}",
                error, &user_id
            ),
        };
        return Ok(json(&response));
    }
    let result = manager.join_habits_recurrences_data(result.unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!(
                "Error getting habits, recurrences & data: {} for user with ID: {}",
                error, &user_id
            ),
        };
        return Ok(json(&response));
    }
    // Return response
    let response = HabitsAndRecurrencesAndDataQueryResponse {
        message: format!(
            "Successfully retrieved habits, recurrences & data for user with ID: {}",
            &user_id
        ),
        habits: result.unwrap(),
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_habits_handler(manager: DBManager, id: Uuid) -> Result<impl Reply, Rejection> {
    // Delete habit from database
    let result = manager.delete_habit(id);

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
    manager: DBManager,
    id: Uuid,
    data: HabitCreateSchema,
) -> Result<impl Reply, Rejection> {
    let result = manager.update_habit(id, data);

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
