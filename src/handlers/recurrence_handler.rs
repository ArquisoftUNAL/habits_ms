use crate::{
    db::DBManager,
    error::Error,
    models::api::{recurrence_api_models::*, *},
    utils::queries::join_recurrence_with_data,
};

use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Rejection, Reply,
};

use uuid::Uuid;
use validator::Validate;

// POST Route
pub async fn create_recurrence_handler(
    manager: DBManager,
    data: RecurrenceCreateSchema,
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
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = RecurrenceCreateResponse {
        message: "Recurrence created successfully".to_string(),
        id: result.unwrap(),
    };
    return Ok(with_status(json(&response), StatusCode::CREATED));
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
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = GeneralResponse {
        message: "Recurrence updated successfully".to_string(),
    };

    return Ok(with_status(json(&response), StatusCode::OK));
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
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = GeneralResponse {
        message: "Recurrence deleted successfully".to_string(),
    };
    return Ok(with_status(json(&response), StatusCode::OK));
}

// GET Route
pub async fn get_recurrence_by_id_handler(
    id: Uuid,
    params: RangeParams,
    manager: DBManager,
    data_params: DataIncludeParams,
) -> Result<impl Reply, Rejection> {
    let result = manager.get_recurrence_by_id(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    if data_params.include_data.unwrap_or(false) {
        let data_result =
            manager.get_all_recurrence_data(id, params.data_page, params.data_per_page);

        if data_result.is_err() {
            let error = data_result.err().unwrap();
            return Err(warp::reject::custom(error));
        }

        let data_result = data_result.unwrap();

        // Return response
        let response = RecurrencesWithDataSingleQueryResponse {
            message: format!("Successfully retrieved recurrence data"),
            recurrence: join_recurrence_with_data(result, data_result),
        };

        return Ok(with_status(json(&response), StatusCode::OK));
    }

    // Return response
    let response = RecurrencesSingleQueryResponse {
        message: format!("Successfully retrieved recurrence"),
        recurrence: result,
    };

    return Ok(with_status(json(&response), StatusCode::OK));
}
