use crate::{
    db::DBManager,
    error::Error,
    models::api::{recurrency_api_models::*, *},
};

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;
use validator::Validate;

// POST Route
pub async fn create_recurrency_handler(
    manager: DBManager,
    data: RecurrencyCreateSchema,
) -> Result<impl Reply, Rejection> {
    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }
    // Create model from request body
    let result = manager.add_recurrence(data);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error creating habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = RecurrenceCreateResponse {
        message: "Recurrence created successfully".to_string(),
        id: result.unwrap(),
    };
    Ok(json(&response))
}

// GET Route
pub async fn get_habit_recurrences_handler(
    manager: DBManager,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let result = manager.get_all_habit_recurrences(id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error getting habit's recurrences: {}", error),
        };
        return Ok(json(&response));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    // Return response
    let response = RecurrencesMultipleQueryResponse {
        message: format!("Successfully retrieved habit's recurrences"),
        recurrences: result,
    };

    Ok(json(&response))
}

// GET Route
pub async fn get_recurrence_by_id_handler(
    manager: DBManager,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    let result = manager.get_recurrence_by_id(id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error getting recurrence: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = RecurrencesSingleQueryResponse {
        message: format!("Successfully retrieved recurrence"),
        recurrence: result.unwrap(),
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_recurrence_handler(
    manager: DBManager,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    // Delete habit from database
    let result = manager.delete_recurrence(id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error deleting recurrency: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        message: "Recurrency deleted successfully".to_string(),
    };
    Ok(json(&response))
}

// UPDATE (PATCH) Route
pub async fn update_recurrence_handler(
    manager: DBManager,
    id: Uuid,
    data: RecurrenceUpdateSchema,
) -> Result<impl Reply, Rejection> {
    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    let result = manager.update_recurrence(id, data);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error updating recurrency: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        message: "Recurrency updated successfully".to_string(),
    };

    Ok(json(&response))
}
