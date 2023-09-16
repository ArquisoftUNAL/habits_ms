use crate::{
    db::DBManager,
    error::Error,
    models::api::{data_api_models::*, *},
};

use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Rejection, Reply,
};

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
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = HabitDataCreateResponse {
        message: "Habit data created successfully".to_string(),
        id: result.unwrap(),
    };
    Ok(with_status(json(&response), StatusCode::CREATED))
}

// GET Route
pub async fn get_data_by_id_handler(manager: DBManager, id: Uuid) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let result = manager.get_habit_data_by_id(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    // Return response
    let response = HabitDataSingleQueryResponse {
        message: format!("Successfully retrieved habit data"),
        habit_data: result,
    };

    Ok(with_status(json(&response), StatusCode::OK))
}

// DELETE Route
pub async fn delete_habit_data_handler(
    manager: DBManager,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    let result = manager.delete_habit_data(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = GeneralResponse {
        message: "Habit data deleted successfully".to_string(),
    };
    Ok(with_status(json(&response), StatusCode::OK))
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

    let result = manager.update_habit_data(id, data);

    if result.is_err() {
        return Err(warp::reject::custom(result.err().unwrap()));
    }

    // Return response
    let response = GeneralResponse {
        message: "Habit data updated successfully".to_string(),
    };

    Ok(with_status(json(&response), StatusCode::OK))
}
