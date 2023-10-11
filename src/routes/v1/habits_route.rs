use crate::{
    db::PostgresPool,
    handlers::habit_handler,
    models::api::{DataIncludeParams, DateParams, RangeParams},
    utils::{with_authenticator, with_db_manager},
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
        .and(with_authenticator())
        .and(warp::body::json())
        .and_then(habit_handler::create_habit_handler);

    let update_habit = base_habit_route
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::path::param::<Uuid>())
        .and(warp::body::json())
        .and_then(habit_handler::update_habits_handler);

    let delete_habit = base_habit_route
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::path::param::<Uuid>())
        .and_then(habit_handler::delete_habits_handler);

    // Getting habits by user id
    let base_get_habit_route = base_habit_route
        .and(warp::get())
        .and(warp::query::<RangeParams>());

    let get_habits = base_get_habit_route
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::any().map(move || DataIncludeParams {
            ..Default::default()
        }))
        .and(warp::path::end())
        .and(warp::query::<DateParams>())
        .and_then(habit_handler::get_habits_by_user_id_handler);

    let get_habits_data = base_get_habit_route
        .and(warp::path("data"))
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::any().map(move || DataIncludeParams {
            include_data: Some(true),
            ..Default::default()
        }))
        .and(warp::query::<DateParams>())
        .and_then(habit_handler::get_habits_by_user_id_handler);

    // Getting habits by category id
    let get_habit_by_category = base_habit_route
        .and(warp::get())
        .and(warp::path("category"))
        .and(warp::path::param::<Uuid>())
        .and(warp::query::<RangeParams>())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and_then(habit_handler::get_habits_by_category_handler);

    // Getting habits by id
    let base_get_habit_id_route = base_habit_route
        .and(warp::get())
        .and(warp::path::param::<Uuid>())
        .and(warp::query::<RangeParams>());

    let get_habit_by_id = base_get_habit_id_route
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::any().map(move || DataIncludeParams {
            ..Default::default()
        }))
        .and(warp::path::end())
        .and_then(habit_handler::get_habit_by_id_handler);

    let get_habit_by_id_data = base_get_habit_id_route
        .and(warp::path("data"))
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::any().map(move || DataIncludeParams {
            include_data: Some(true),
            ..Default::default()
        }))
        .and_then(habit_handler::get_habit_by_id_handler);

    create_habit
        .or(update_habit)
        .or(delete_habit)
        .or(get_habits)
        .or(get_habits_data)
        .or(get_habit_by_category)
        .or(get_habit_by_id)
        .or(get_habit_by_id_data)
        .boxed()
}
