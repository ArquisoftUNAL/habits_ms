use crate::{
    db::PostgresPool,
    handlers::events_handler,
    models::api::{DateParams, RangeParams},
    utils::with_db_manager,
};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let base_events_route = warp::path("events");

    // Allow to get next ocurrences from habit
    let get_next_end_event_by_habit = base_events_route
        .and(warp::get())
        .and(warp::path("habit"))
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::query::<DateParams>())
        .and(warp::query::<RangeParams>())
        .and(with_db_manager(pool.clone()))
        .and_then(events_handler::get_next_events_by_habit_handler);

    get_next_end_event_by_habit.boxed()
}
