use crate::{db::PostgresPool, handlers::habit_handler, utils::with_db_manager};

use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let habits_path = warp::path("habits");

    habits_path
        // Insert an Habit into databases
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(habit_handler::create_habit_handler)
        .or(
            // Get habits from database (for a given user)
            habits_path
                .and(warp::get())
                .and(warp::path("user"))
                .and(with_db_manager(pool.clone()))
                .and(warp::path::param::<String>())
                .and(warp::path::end())
                .and_then(habit_handler::get_habits_handler),
        )
        .or(
            // Get habits from database (for a given user) and join recurrences as well
            habits_path
                .and(warp::get())
                .and(warp::path("user"))
                .and(with_db_manager(pool.clone()))
                .and(warp::path::param::<String>())
                .and(warp::path("recurrences"))
                .and(warp::path::end())
                .and_then(habit_handler::get_habits_recurrences_by_user_id),
        )
        .or(
            // Get habits from database (for a given user) and join recurrences as well
            habits_path
                .and(warp::get())
                .and(warp::path("user"))
                .and(with_db_manager(pool.clone()))
                .and(warp::path::param::<String>())
                .and(warp::path("recurrences"))
                .and(warp::path("data"))
                .and(warp::path::end())
                .and_then(habit_handler::get_habits_recurrences_data_by_user_id),
        )
        .or(
            // Get habits from database (for a given user)
            habits_path
                .and(warp::get())
                .and(warp::path("user"))
                .and(with_db_manager(pool.clone()))
                .and(warp::path::param::<Uuid>())
                .and(warp::path::end())
                .and_then(habit_handler::get_habit_by_id_handler),
        )
        .or(
            // Update an habit from database
            habits_path
                .and(warp::patch())
                .and(with_db_manager(pool.clone()))
                .and(warp::path::param::<Uuid>())
                .and(warp::body::json())
                .and(warp::path::end())
                .and_then(habit_handler::update_habits_handler),
        )
        .or(
            // Delete an habit from database
            habits_path
                .and(warp::delete())
                .and(with_db_manager(pool.clone()))
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::path::end())
                .and_then(habit_handler::delete_habits_handler),
        )
        .boxed()
}
