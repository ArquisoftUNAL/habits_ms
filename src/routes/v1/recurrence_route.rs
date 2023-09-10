use crate::{db::PostgresPool, handlers::recurrence_handler, utils::with_db_manager};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let update_recurrence = warp::path("recurrences")
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::body::json())
        .and_then(recurrence_handler::update_recurrence_handler);

    let create_recurrence = warp::path("recurrences")
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(warp::body::json())
        .and_then(recurrence_handler::create_recurrence_handler);

    let get_habit_recurrences = warp::path("recurrences")
        .and(warp::path("habit"))
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<uuid::Uuid>())
        .and_then(recurrence_handler::get_habit_recurrences_handler);

    let get_recurrence_by_id = warp::path("recurrences")
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<uuid::Uuid>())
        .and_then(recurrence_handler::get_recurrence_by_id_handler);

    let delete_recurrence = warp::path("recurrences")
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<uuid::Uuid>())
        .and_then(recurrence_handler::delete_recurrence_handler);

    update_recurrence
        .or(create_recurrence)
        .or(get_habit_recurrences)
        .or(get_recurrence_by_id)
        .or(delete_recurrence)
        .boxed()
}
