use crate::{
    db::DBManager,
    error::Error,
    models::{
        api::{habit_api_models::*, *},
        database::Habit,
    },
    utils::queries::join_habit_with_data,
};

use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Rejection, Reply,
};

use uuid::Uuid;
use validator::Validate;

// POST Route
pub async fn create_habit_handler(
    manager: DBManager,
    authentication: AuthData,
    data: HabitCreateSchema,
) -> Result<impl Reply, Rejection> {
    // Check a user is logged in / provided the action
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    let user_id = authentication.requester_id;

    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    // Create model from request body
    let result = manager.add_habit(user_id, data);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = HabitCreateResponse {
        message: format!("Habit created successfully"),
        id: result.unwrap(),
    };

    Ok(with_status(json(&response), StatusCode::CREATED))
}

// UPDATE (PATCH) Route
pub async fn update_habits_handler(
    manager: DBManager,
    authentication: AuthData,
    id: Uuid,
    data: HabitUpdateSchema,
) -> Result<impl Reply, Rejection> {
    // Check a user is logged in / provided the action
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    // Check if habit is accessible by user
    let result = manager.is_habit_accessible_by_user(authentication.requester_id, id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User has not access to this habit".to_string(),
        )));
    }

    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    let result = manager.update_habit(id, data);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = GeneralResponse {
        message: "Habit updated successfully".to_string(),
    };

    Ok(with_status(json(&response), StatusCode::OK))
}

// DELETE Route
pub async fn delete_habits_handler(
    manager: DBManager,
    authentication: AuthData,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    // Check a user is logged in / provided the action
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    // Check if habit is accessible by user
    let result = manager.is_habit_accessible_by_user(authentication.requester_id, id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User has not access to this habit".to_string(),
        )));
    }

    // Delete habit from database
    let result = manager.delete_habit(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = GeneralResponse {
        message: "Habit deleted successfully".to_string(),
    };
    Ok(with_status(json(&response), StatusCode::OK))
}

// GET Route
pub async fn get_habits_by_user_id_handler(
    params: RangeParams,
    manager: DBManager,
    authentication: AuthData,
    data_params: DataIncludeParams,
    date_params: DateParams,
) -> Result<impl Reply, Rejection> {
    // Check a user is logged in / provided the action
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    let user_id = authentication.requester_id;

    let result = manager.get_all_user_habits(user_id, params.habits_page, params.habits_per_page);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    // Join data case
    if data_params.include_data.unwrap_or(false) {
        let result = manager.join_habits_data(result, date_params.start_date, date_params.end_date);

        if result.is_err() {
            let error = result.err().unwrap();
            return Err(warp::reject::custom(error));
        }

        // Return response
        let response = HabitAndDataMultipleQueryResponse {
            message: format!("Successfully retrieved habits & data for user"),
            habits: result.unwrap(),
        };

        return Ok(with_status(json(&response), StatusCode::OK));
    }

    // Only habits case
    // Return response
    let response = HabitMultipleQueryResponse {
        message: format!("Successfully retrieved habits for user"),
        habits: result,
    };

    return Ok(with_status(json(&response), StatusCode::OK));
}

pub async fn get_habits_by_category_handler(
    id: Uuid,
    params: RangeParams,
    manager: DBManager,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    let category_id = id.clone();
    let user_id = authentication.requester_id;
    let result: Result<Vec<Habit>, Error>;

    if user_id == "admin".to_string() {
        result = manager.get_all_category_habits(
            category_id,
            params.habits_page,
            params.habits_per_page,
        );
    } else {
        result = manager.get_all_user_category_habits(
            user_id,
            category_id,
            params.habits_page,
            params.habits_per_page,
        );
    }

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    // Return response
    let response = HabitMultipleQueryResponse {
        message: format!("Successfully retrieved habits for category"),
        habits: result,
    };

    return Ok(with_status(json(&response), StatusCode::OK));
}

// GET Route
pub async fn get_habit_by_id_handler(
    id: Uuid,
    params: RangeParams,
    manager: DBManager,
    authentication: AuthData,
    data_params: DataIncludeParams,
) -> Result<impl Reply, Rejection> {
    // Check a user is logged in / provided the action
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    // Check if habit is accessible by user
    let result = manager.is_habit_accessible_by_user(authentication.requester_id, id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User has not access to this habit".to_string(),
        )));
    }

    // Get habits from database
    let result = manager.get_habit_by_id(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    // Join data case
    if data_params.include_data.unwrap_or(false) {
        let data =
            manager.get_all_habit_data(id, None, None, params.data_page, params.data_per_page);

        if data.is_err() {
            let error = data.err().unwrap();
            return Err(warp::reject::custom(error));
        }

        let data = data.unwrap();

        // Return response
        let response = HabitAndDataSingleQueryResponse {
            message: format!("Successfully retrieved habit & recurrences & data"),
            habit: join_habit_with_data(result, data),
        };

        return Ok(with_status(json(&response), StatusCode::OK));
    }

    // Return response
    let response = HabitSingleQueryResponse {
        message: format!("Successfully retrieved habit"),
        habit: result,
    };

    return Ok(with_status(json(&response), StatusCode::OK));
}
