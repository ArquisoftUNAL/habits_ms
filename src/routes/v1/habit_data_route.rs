use crate::{
    db::PostgresPool,
    handlers::habit_data_handler,
    models::api::{AdminParams, RangeParams},
    utils::{with_authenticator, with_db_manager},
};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use uuid::Uuid;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let base_habit_data_route = warp::path("habitdata");

    let create_habit_data = base_habit_data_route
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::body::json())
        .and_then(habit_data_handler::create_habit_data_handler);

    let update_habit_data = base_habit_data_route
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::path::param::<Uuid>())
        .and(warp::body::json())
        .and_then(habit_data_handler::update_habit_data_handler);

    let delete_habit_data = base_habit_data_route
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::path::param::<Uuid>())
        .and_then(habit_data_handler::delete_habit_data_handler);

    // Querying habit data
    let get_habit_data_by_user = base_habit_data_route
        .and(warp::get())
        .and(warp::query::<RangeParams>())
        .and(warp::query::<AdminParams>())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::path::end())
        .and_then(habit_data_handler::get_data_by_user_handler);

    let get_habit_data_by_habit = base_habit_data_route
        .and(warp::get())
        .and(warp::path("habit"))
        .and(warp::path::param::<Uuid>())
        .and(warp::query::<RangeParams>())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and_then(habit_data_handler::get_data_by_habit_handler);

    let get_habit_data_by_id = base_habit_data_route
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::path::param::<Uuid>())
        .and_then(habit_data_handler::get_data_by_id_handler);

    create_habit_data
        .or(update_habit_data)
        .or(delete_habit_data)
        .or(get_habit_data_by_user)
        .or(get_habit_data_by_habit)
        .or(get_habit_data_by_id)
        .boxed()
}
