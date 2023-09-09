use crate::{db::PostgresPool, handlers::habit_handler, utils::with_db_manager};

use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let create_habit = warp::path("habits")
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(warp::body::json())
        .and_then(habit_handler::create_habit_handler);

    let get_habit_by_id = warp::path("habits")
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and_then(habit_handler::get_habit_by_id_handler);

    let get_habits = warp::path("habits")
        .and(warp::path("user"))
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(habit_handler::get_habits_handler);

    let get_habits_recurrences = warp::path("habits")
        .and(warp::path("user"))
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<String>())
        .and(warp::path("recurrences"))
        .and(warp::path::end())
        .and_then(habit_handler::get_habits_recurrences_by_user_id);

    let get_habits_recurrences_data = warp::path("habits")
        .and(warp::path("user"))
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<String>())
        .and(warp::path("recurrences"))
        .and(warp::path("data"))
        .and_then(habit_handler::get_habits_recurrences_data_by_user_id);

    let update_habit = warp::path("habits")
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::json())
        .and_then(habit_handler::update_habits_handler);

    let delete_habit = warp::path("habits")
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and_then(habit_handler::delete_habits_handler);

    get_habits
        .or(create_habit)
        .or(get_habits_recurrences)
        .or(get_habits_recurrences_data)
        .or(get_habit_by_id)
        .or(update_habit)
        .or(delete_habit)
        .boxed()
}
