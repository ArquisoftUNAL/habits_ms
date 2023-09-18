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
    let habits =
        manager.get_all_user_habits(id, range_params.habits_page, range_params.habits_per_page);

    if habits.is_err() {
        let error = habits.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let habits = habits.unwrap();

    if data_params.include_habits.unwrap_or(false) {
        let events_data = manager.get_next_events_with_habits(
            habits,
            date_params.start_date,
            date_params.end_date,
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

    // Return response
    let response = GeneralResponse {
        message: format!("Successfully retrieved recurrences"),
    };

    return Ok(with_status(json(&response), StatusCode::OK));
}
