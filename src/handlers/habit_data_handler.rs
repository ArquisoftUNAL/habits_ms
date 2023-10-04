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
    data: HabitDataCreateSchema,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check if user is logged in
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    let user_id = authentication.requester_id;

    // Check if user is the owner of the habit
    let result = manager.is_habit_accessible_by_user(user_id, data.habit_id);

    if result.is_err() {
        return Err(warp::reject::custom(result.err().unwrap()));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User is not the owner of the habit".to_string(),
        )));
    }

    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    // Check if requested date is strictly after the last habit's data
    let last_habit_data = manager.get_all_habit_data(data.habit_id, Some(1), Some(1));

    if last_habit_data.is_err() {
        return Err(warp::reject::custom(last_habit_data.err().unwrap()));
    }

    let last_habit_data = last_habit_data.unwrap();

    if last_habit_data.len() > 0 && data.collected_at.is_some() {
        if last_habit_data[0].hab_dat_collected_at > data.collected_at.unwrap() {
            return Err(warp::reject::custom(Error::BadRequest(
                "Requested date is before the last habit's data".to_string(),
            )));
        }
    }

    // Create model from request body
    let result = manager.add_habit_data(data);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let data = result.unwrap();

    // Return response
    let response = HabitDataCreateResponse {
        message: "Habit data created successfully".to_string(),
        data,
    };

    Ok(with_status(json(&response), StatusCode::CREATED))
}

// UPDATE (PATCH) Route
pub async fn update_habit_data_handler(
    manager: DBManager,
    id: Uuid,
    data: HabitDataUpdateSchema,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check if user is logged in
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    // Check if habit is accessible by user
    let result = manager.is_habitdata_accessible_by_user(authentication.requester_id, id);

    if result.is_err() {
        return Err(warp::reject::custom(result.err().unwrap()));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User is not the owner of the habit data".to_string(),
        )));
    }

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
    let response = HabitDataUpdateDeleteResponse {
        message: "Habit data updated successfully".to_string(),
        data: result.unwrap(),
    };

    Ok(with_status(json(&response), StatusCode::OK))
}

// DELETE Route
pub async fn delete_habit_data_handler(
    manager: DBManager,
    id: Uuid,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check if user is logged in
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    // Check if habit is accessible by user
    let result = manager.is_habitdata_accessible_by_user(authentication.requester_id, id);

    if result.is_err() {
        return Err(warp::reject::custom(result.err().unwrap()));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User is not the owner of the habit data".to_string(),
        )));
    }

    let result = manager.delete_habit_data(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = HabitDataUpdateDeleteResponse {
        message: "Habit data deleted successfully".to_string(),
        data: result.unwrap(),
    };
    Ok(with_status(json(&response), StatusCode::OK))
}

// GET Route
pub async fn get_data_by_id_handler(
    manager: DBManager,
    id: Uuid,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check if user is logged in
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    // Check if habit is accessible by user
    let result = manager.is_habitdata_accessible_by_user(authentication.requester_id, id);

    if result.is_err() {
        return Err(warp::reject::custom(result.err().unwrap()));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User is not the owner of the habit data".to_string(),
        )));
    }

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

// GET Route
pub async fn get_data_by_habit_handler(
    id: Uuid,
    params: RangeParams,
    manager: DBManager,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check if user is logged in
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    // Check if habit is accessible by user
    let result = manager.is_habit_accessible_by_user(authentication.requester_id, id);

    if result.is_err() {
        return Err(warp::reject::custom(result.err().unwrap()));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User is not the owner of the habit".to_string(),
        )));
    }

    // Get habits from database
    let result = manager.get_all_habit_data(id, params.data_page, params.data_per_page);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    // Return response
    let response = HabitDataMultipleQueryResponse {
        message: format!("Successfully retrieved habit data"),
        habit_data: result,
    };

    Ok(with_status(json(&response), StatusCode::OK))
}

// GET Route
pub async fn get_data_by_user_handler(
    params: RangeParams,
    manager: DBManager,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check a user is logged in / provided the action
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    let user_id = authentication.requester_id;

    // Get habits from database
    let result = manager.get_all_user_habitdata(user_id, params.data_page, params.data_per_page);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    // Return response
    let response = HabitDataMultipleQueryResponse {
        message: format!("Successfully retrieved user's habit data"),
        habit_data: result,
    };

    Ok(with_status(json(&response), StatusCode::OK))
}
