use crate::{
    db::DBManager,
    models::api::{events_api_models::*, *},
};

use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Rejection, Reply,
};

// GET Route
pub async fn get_next_events_by_user_handler(
    id: String,
    date_params: DateParams,
    range_params: RangeParams,
    manager: DBManager,
    data_params: DataIncludeParams,
) -> Result<impl Reply, Rejection> {
    if data_params.include_habits.unwrap_or(false) {
        let events_data = manager.get_next_events_with_habits(
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

        let response = EventsHabitsMultipleQueryResponse {
            message: format!("Successfully retrieved events"),
            events: events_data,
        };

        return Ok(with_status(json(&response), StatusCode::OK));
    }

    let events_data =
        manager.get_next_events_counts(id, date_params.start_date, date_params.end_date);

    if events_data.is_err() {
        let error = events_data.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let events_data = events_data.unwrap();

    let response = EventsCountMultipleQueryResponse {
        message: format!("Successfully retrieved events"),
        events: events_data,
    };

    return Ok(with_status(json(&response), StatusCode::OK));
}

// GET Route
pub async fn get_next_events_by_habit_handler(
    id: uuid::Uuid,
    date_params: DateParams,
    range_params: RangeParams,
    manager: DBManager,
) -> Result<impl Reply, Rejection> {
    let events_data = manager.get_next_events_by_habit(
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

    let response = EventsRecurrencesMultipleQueryResponse {
        message: format!("Successfully retrieved events"),
        events: events_data,
    };

    return Ok(with_status(json(&response), StatusCode::OK));
}
