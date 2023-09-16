use crate::{
    db::PostgresPool,
    handlers::habit_handler,
    models::api::{DataIncludeParams, RangeParams},
    utils::with_db_manager,
};

use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let base_habit_route = warp::path("habits");

    let create_habit = base_habit_route
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(warp::body::json())
        .and_then(habit_handler::create_habit_handler);

    let update_habit = base_habit_route
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::json())
        .and_then(habit_handler::update_habits_handler);

    let delete_habit = base_habit_route
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and_then(habit_handler::delete_habits_handler);

    // Getting habits by user id
    let base_get_habit_user_route = base_habit_route
        .and(warp::get())
        .and(warp::path("user"))
        .and(warp::path::param::<String>())
        .and(warp::query::<RangeParams>());

    let get_habits = base_get_habit_user_route
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            ..Default::default()
        }))
        .and(warp::path::end())
        .and_then(habit_handler::get_habits_handler_by_user_id_handler);

    let get_habits_recurrences = base_get_habit_user_route
        .and(warp::path("recurrences"))
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            include_recurrences: Some(true),
            ..Default::default()
        }))
        .and(warp::path::end())
        .and_then(habit_handler::get_habits_handler_by_user_id_handler);

    let get_habits_recurrences_data = base_get_habit_user_route
        .and(warp::path("recurrences"))
        .and(warp::path("data"))
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            include_data: Some(true),
            ..Default::default()
        }))
        .and_then(habit_handler::get_habits_handler_by_user_id_handler);

    // Getting habits by id
    let base_get_habit_id_route = base_habit_route
        .and(warp::get())
        .and(warp::path::param::<Uuid>())
        .and(warp::query::<RangeParams>());

    let get_habit_by_id = base_get_habit_id_route
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            ..Default::default()
        }))
        .and(warp::path::end())
        .and_then(habit_handler::get_habit_by_id_handler);

    let get_habit_by_id_recurrences = base_get_habit_id_route
        .and(warp::path("recurrences"))
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            include_recurrences: Some(true),
            ..Default::default()
        }))
        .and(warp::path::end())
        .and_then(habit_handler::get_habit_by_id_handler);

    let get_habit_by_id_recurrences_data = base_get_habit_id_route
        .and(warp::path("recurrences"))
        .and(warp::path("data"))
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            include_data: Some(true),
            ..Default::default()
        }))
        .and_then(habit_handler::get_habit_by_id_handler);

    create_habit
        .or(update_habit)
        .or(delete_habit)
        .or(get_habits)
        .or(get_habits_recurrences)
        .or(get_habits_recurrences_data)
        .or(get_habit_by_id)
        .or(get_habit_by_id_recurrences)
        .or(get_habit_by_id_recurrences_data)
        .boxed()
}
