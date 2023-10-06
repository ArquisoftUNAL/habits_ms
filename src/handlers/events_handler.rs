use crate::{
    db::DBManager,
    error::Error,
    models::api::{events_api_models::*, *},
};

use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Rejection, Reply,
};

use uuid::Uuid;

// GET Route
pub async fn get_next_events_by_habit_handler(
    id: Uuid,
    date_params: DateParams,
    range_params: RangeParams,
    manager: DBManager,
) -> Result<impl Reply, Rejection> {
    let events_data = manager.get_habit_next_events(
        id,
        date_params.start_date,
        date_params.end_date,
        range_params.events_limit,
    );

    if events_data.is_err() {
        let error = events_data.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let events_data = events_data.unwrap();

    let response = EventsMultipleQueryResponse {
        message: format!("Successfully retrieved events"),
        events: events_data,
    };

    return Ok(with_status(json(&response), StatusCode::OK));
}

// GET Route
pub async fn get_data_by_habit_handler(
    date_params: DateParams,
    id: Uuid,
    manager: DBManager,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check if user is logged in
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    let user_id = authentication.requester_id;

    // Check if habit is accessible by user
    let result = manager.is_habit_accessible_by_user(user_id.clone(), id);

    if result.is_err() {
        return Err(warp::reject::custom(result.err().unwrap()));
    }

    if !result.unwrap() {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "User is not the owner of the habit".to_string(),
        )));
    }

    // Get habits from database
    let result = manager.get_habitdata_as_calendar(
        Some(user_id),
        Some(id),
        date_params.start_date,
        date_params.end_date,
    );

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    // Return response
    let response = CalendarEventsMultipleQueryResponse {
        message: format!("Successfully retrieved habit data as calendar events"),
        events: result,
    };

    Ok(with_status(json(&response), StatusCode::OK))
}

// GET Route
pub async fn get_data_by_user_handler(
    date_params: DateParams,
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
    let result = manager.get_habitdata_as_calendar(
        Some(user_id),
        None,
        date_params.start_date,
        date_params.end_date,
    );

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let result = result.unwrap();

    // Return response
    let response = CalendarEventsMultipleQueryResponse {
        message: format!("Successfully retrieved habit data as calendar events"),
        events: result,
    };

    Ok(with_status(json(&response), StatusCode::OK))
}
