use crate::{db::PostgresPool, handlers::recurrency_handler, utils::with_db_manager};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let recurrences_path = warp::path("recurrences");
    recurrences_path
        // Insert an Habit into databases
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(recurrency_handler::create_recurrency_handler)
        .or(
            // Get recurrences from database (for a given habit)
            recurrences_path
                .and(warp::get())
                .and(warp::path("habit"))
                .and(with_db_manager(pool.clone()))
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::path::end())
                .and_then(recurrency_handler::get_habit_recurrences_handler),
        )
        // .or(
        //     // Get a specific recurrence by ID
        //     recurrences_path
        //         .and(warp::get())
        //         .and(with_db_manager(pool.clone()))
        //         .and(warp::path::param::<uuid::Uuid>())
        //         .and(warp::path::end())
        //         .and_then(recurrency_handler::get_recurrence_by_id_handler),
        // )
        // .or(
        //     // Update an habit from database
        //     recurrences_path
        //         .and(warp::patch())
        //         .and(with_db_manager(pool.clone()))
        //         .and(warp::path::param::<uuid::Uuid>())
        //         .and(warp::body::json())
        //         .and_then(recurrency_handler::update_recurrence_handler),
        // )
        // .or(
        //     // Delete an habit from database
        //     recurrences_path
        //         .and(warp::delete())
        //         .and(with_db_manager(pool.clone()))
        //         .and(warp::path::param::<uuid::Uuid>())
        //         .and_then(recurrency_handler::delete_recurrence_handler),
        // )
        .boxed()
}
