use crate::{db::PostgresPool, handlers::habit_data_handler, utils::with_db_manager};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use uuid::Uuid;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let create_habit_data = warp::path("habitdata")
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(warp::body::json())
        .and_then(habit_data_handler::create_habit_data_handler);

    let get_habit_data_by_id = warp::path("habitdata")
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and_then(habit_data_handler::get_data_by_id_handler);

    let get_habit_data_recurrence = warp::path("habitdata")
        .and(warp::get())
        .and(warp::path("recurrence"))
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and_then(habit_data_handler::get_recurrency_data_handler);

    let update_habit_data = warp::path("habitdata")
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::json())
        .and_then(habit_data_handler::update_habit_data_handler);

    let delete_habit_data = warp::path("habitdata")
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and_then(habit_data_handler::delete_habit_data_handler);

    create_habit_data
        .or(get_habit_data_by_id)
        .or(get_habit_data_recurrence)
        .or(update_habit_data)
        .or(delete_habit_data)
        .boxed()
}
