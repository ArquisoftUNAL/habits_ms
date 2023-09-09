use crate::{
    db::DBManager,
    error::Error,
    models::api::{data_api_models::*, *},
};

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;
use validator::Validate;

// POST Route
pub async fn create_habit_data_handler(
    manager: DBManager,
    data: HabitDataSchema,
) -> Result<impl Reply, Rejection> {
    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    // Create model from request body
    let result = manager.add_habit_data(data);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error creating habit data: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = HabitDataCreateResponse {
        message: "Habit data created successfully".to_string(),
        id: result.unwrap(),
    };
    Ok(json(&response))
}

// GET Route
pub async fn get_recurrency_data_handler(
    manager: DBManager,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let result = manager.get_all_recurrency_data(id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error getting recurrency's habit data: {}", error),
        };
        return Ok(json(&response));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    // Return response
    let response = HabitDataMultipleQueryResponse {
        message: format!("Successfully retrieved recurrency's habit data"),
        habit_data: result,
    };

    Ok(json(&response))
}

// GET Route
pub async fn get_data_by_id_handler(manager: DBManager, id: Uuid) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let result = manager.get_habit_data_by_id(id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error getting habit data: {}", error),
        };
        return Ok(json(&response));
    }

    let result = result.unwrap();

    // Return response
    let response = HabitDataSingleQueryResponse {
        message: format!("Successfully retrieved habit data"),
        habit_data: result,
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_habit_data_handler(
    manager: DBManager,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    let result = manager.delete_habit_data(id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error deleting habit data: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        message: "Habit data deleted successfully".to_string(),
    };
    Ok(json(&response))
}

// UPDATE (PATCH) Route
pub async fn update_habit_data_handler(
    manager: DBManager,
    id: Uuid,
    data: HabitDataUpdateSchema,
) -> Result<impl Reply, Rejection> {
    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    // First check if habit exists
    let result = manager.get_habit_data_by_id(id);

    let mut updatable_data = data;

    if !result.is_err() {
        // Load data recurrence and belongs to habit
        let existent_data = result.unwrap();

        let result = manager.get_parent_recurrency_and_habit(&existent_data);

        if result.is_err() {
            return Err(warp::reject::custom(result.err().unwrap()));
        }

        let (_, habit) = result.unwrap();

        updatable_data = HabitDataUpdateSchema { ..updatable_data };

        if !habit.hab_is_yn {
            // Update amount rather than reassign it
            updatable_data.amount += existent_data.hab_dat_amount;
        }
    }

    let result = manager.update_habit_data(id, updatable_data);

    if result.is_err() {
        return Err(warp::reject::custom(result.err().unwrap()));
    }

    // Return response
    let response = GeneralResponse {
        message: "Habit data updated successfully".to_string(),
    };

    Ok(json(&response))
}
