use crate::controllers::habit_handler;

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    let habits_path = warp::path("habits");
    habits_path
        // Insert an Habit into databases
        .and(warp::any())
        .and(warp::body::json())
        .and_then(habit_handler::create_habit_handler)
        .or(
            // Get habits from database (for a given user)
            habits_path
                .and(warp::get())
                .and(warp::path::param::<String>())
                .and_then(habit_handler::get_habits_handler),
        )
        .or(
            // Update an habit from database
            habits_path
                .and(warp::patch())
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::body::json())
                .and_then(habit_handler::update_habits_handler),
        )
        .or(
            // Delete an habit from database
            habits_path
                .and(warp::delete())
                .and(warp::path::param::<uuid::Uuid>())
                .and_then(habit_handler::delete_habits_handler),
        )
        .boxed()
}
