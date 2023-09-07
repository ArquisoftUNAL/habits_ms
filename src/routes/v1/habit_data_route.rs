use crate::controllers::habit_data_handler;

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    let habit_data_path = warp::path("habitdata");
    habit_data_path
        // Insert an Habit into databases
        .and(warp::post())
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::body::json())
        .and_then(habit_data_handler::create_habit_data_handler)
        .or(
            // Get habits from database (for a given user)
            habit_data_path
                .and(warp::get())
                .and(warp::path::param::<uuid::Uuid>())
                .and_then(habit_data_handler::get_habit_data_handler),
        )
        .or(
            // Update an habit from database
            habit_data_path
                .and(warp::patch())
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::body::json())
                .and_then(habit_data_handler::update_habit_data_handler),
        )
        .or(
            // Delete an habit from database
            habit_data_path
                .and(warp::delete())
                .and(warp::path::param::<uuid::Uuid>())
                .and_then(habit_data_handler::delete_habit_data_handler),
        )
        .boxed()
}
