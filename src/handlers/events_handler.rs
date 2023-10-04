use crate::{
    db::DBManager,
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
