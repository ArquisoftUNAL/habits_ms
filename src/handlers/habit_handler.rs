use crate::{
    db::DBManager,
    error::Error,
    models::api::{habit_api_models::*, *},
    utils::queries::{join_habit_recurrence_and_data, join_habit_with_recurrences},
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
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    // Create model from request body
    let result = manager.add_habit(data);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    // Return response
    let response = HabitCreateResponse {
        message: format!("Habit created successfully"),
        id: result.unwrap(),
    };

    Ok(json(&response))
}

// UPDATE (PATCH) Route
pub async fn update_habits_handler(
    manager: DBManager,
    id: Uuid,
    data: HabitUpdateSchema,
) -> Result<impl Reply, Rejection> {
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

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_habits_handler(manager: DBManager, id: Uuid) -> Result<impl Reply, Rejection> {
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
    Ok(json(&response))
}

// GET Route
pub async fn get_habits_handler_by_user_id_handler(
    id: String,
    params: RangeParams,
    manager: DBManager,
    data_params: DataIncludeParams,
) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let user_id = id.clone();

    let result = manager.get_all_user_habits(&user_id, params.habits_page, params.habits_per_page);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    // Join recurrences case
    if data_params.include_recurrences.unwrap_or(false) {
        let result = manager.join_habits_recurrences(result);

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

        return Ok(json(&response));
    }

    // Join data case
    if data_params.include_data.unwrap_or(false) {
        let result = manager.join_habits_recurrences_data(result);

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
        let response = HabitsAndRecurrencesAndDataMultipleQueryResponse {
            message: format!(
                "Successfully retrieved habits, recurrences & data for user with ID: {}",
                &user_id
            ),
            habits: result.unwrap(),
        };

        return Ok(json(&response));
    }

    // Only habits case
    // Return response
    let response = HabitMultipleQueryResponse {
        message: format!(
            "Successfully retrieved habits for user with ID: {}",
            &user_id
        ),
        habits: result,
    };

    return Ok(json(&response));
}

// GET Route
pub async fn get_habit_by_id_handler(
    id: Uuid,
    params: RangeParams,
    manager: DBManager,
    data_params: DataIncludeParams,
) -> Result<impl Reply, Rejection> {
    // Get habits from database

    let result = manager.get_habit_by_id(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    // Join recurrences case
    if data_params.include_recurrences.unwrap_or(false) {
        let recurrences = manager.get_all_habit_recurrences(
            id,
            params.recurrences_page,
            params.recurrences_per_page,
        );

        if recurrences.is_err() {
            let error = recurrences.err().unwrap();
            return Err(warp::reject::custom(error));
        }

        let recurrences = recurrences.unwrap();

        // Return response
        let response = HabitAndRecurrencesSingleQueryResponse {
            message: format!("Successfully retrieved habit & recurrences"),
            habit: join_habit_with_recurrences(result, recurrences),
        };

        return Ok(json(&response));
    }

    // Join data case
    if data_params.include_data.unwrap_or(false) {
        let recurrences = manager.get_all_habit_recurrences_data(
            id,
            params.recurrences_page,
            params.recurrences_per_page,
        );

        if recurrences.is_err() {
            let error = recurrences.err().unwrap();
            return Err(warp::reject::custom(error));
        }

        let recurrences = recurrences.unwrap();

        // Return response
        let response = HabitAndRecurrencesAndDataSingleQueryResponse {
            message: format!("Successfully retrieved habit & recurrences & data"),
            habit: join_habit_recurrence_and_data(result, recurrences),
        };

        return Ok(json(&response));
    }

    // Return response
    let response = HabitSingleQueryResponse {
        message: format!("Successfully retrieved habit"),
        habit: result,
    };

    Ok(json(&response))
}
