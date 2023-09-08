use crate::{
    models::api::{recurrency_api_models::*, *},
    queries::recurrences_queries,
};

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;

// POST Route
pub async fn create_recurrency_handler(
    data: RecurrencyCreateSchema,
) -> Result<impl Reply, Rejection> {
    // Create model from request body
    let result = recurrences_queries::add_recurrence(data).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error creating habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        message: "Recurrence created successfully".to_string(),
    };
    Ok(json(&response))
}

// GET Route
pub async fn get_habit_recurrences_handler(id: Uuid) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let result = recurrences_queries::get_all_habit_recurrences(id).await;

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
        habits: result,
    };

    Ok(json(&response))
}

// GET Route
pub async fn get_recurrence_by_id_handler(id: Uuid) -> Result<impl Reply, Rejection> {
    let result = recurrences_queries::get_recurrence_by_id(id).await;

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
        habits: result.unwrap(),
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_recurrence_handler(id: Uuid) -> Result<impl Reply, Rejection> {
    // Delete habit from database
    let result = recurrences_queries::delete_recurrence(id).await;

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
    id: Uuid,
    data: RecurrencyCreateSchema,
) -> Result<impl Reply, Rejection> {
    let result = recurrences_queries::update_recurrence(id, data).await;

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
