use crate::{
    db::PostgresPool,
    handlers::events_handler,
    models::api::{DateParams, RangeParams},
    utils::{with_authenticator, with_db_manager},
};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(
    pool_write: Option<PostgresPool>,
    pool_read: Option<PostgresPool>,
) -> BoxedFilter<(impl Reply,)> {
    let base_events_route = warp::path("events");

    // Allow to get next ocurrences from habit
    let get_next_end_event_by_habit = base_events_route
        .and(warp::get())
        .and(warp::path("habit"))
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::query::<DateParams>())
        .and(warp::query::<RangeParams>())
        .and(with_db_manager(pool_write.clone(), pool_read.clone()))
        .and_then(events_handler::get_next_events_by_habit_handler);

    let base_calendar_route = base_events_route
        .and(warp::get())
        .and(warp::path("calendar"))
        .and(warp::query::<DateParams>());

    let get_calendar_events_by_habit = base_calendar_route
        .and(warp::path("habit"))
        .and(warp::path::param::<uuid::Uuid>())
        .and(with_db_manager(pool_write.clone(), pool_read.clone()))
        .and(with_authenticator())
        .and_then(events_handler::get_data_by_habit_handler);

    let get_calendar_events_by_user = base_calendar_route
        .and(warp::path::end())
        .and(with_db_manager(pool_write.clone(), pool_read.clone()))
        .and(with_authenticator())
        .and_then(events_handler::get_data_by_user_handler);

    get_next_end_event_by_habit
        .or(get_calendar_events_by_habit)
        .or(get_calendar_events_by_user)
        .boxed()
}
