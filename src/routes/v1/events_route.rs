use crate::{
    db::PostgresPool,
    handlers::events_handler,
    models::api::{DataIncludeParams, DateParams, RangeParams},
    utils::with_db_manager,
};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let base_events_route = warp::path("events");

    // Getting next events from recurrences
    let base_get_next_events_by_user = base_events_route
        .and(warp::get())
        .and(warp::path("user"))
        .and(warp::path::param::<String>())
        .and(warp::query::<DateParams>())
        .and(warp::query::<RangeParams>());

    let get_next_events_with_habits = base_get_next_events_by_user
        .and(warp::path("habits"))
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            include_habits: Some(true),
            ..Default::default()
        }))
        .and(warp::path::end())
        .and_then(events_handler::get_next_events_by_user_handler);

    let get_next_events_count = base_get_next_events_by_user
        .and(warp::path("count"))
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            ..Default::default()
        }))
        .and(warp::path::end())
        .and_then(events_handler::get_next_events_by_user_handler);

    // Allow to get next ocurrences from habit
    let get_next_events_by_habit = base_events_route
        .and(warp::get())
        .and(warp::path("habit"))
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::query::<DateParams>())
        .and(warp::query::<RangeParams>())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::end())
        .and_then(events_handler::get_next_events_by_habit_handler);

    get_next_events_count
        .or(get_next_events_with_habits)
        .or(get_next_events_by_habit)
        .boxed()
}
