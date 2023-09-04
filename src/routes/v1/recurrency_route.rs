use crate::controllers::recurrency_handler;

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    let recurrences_path = warp::path("recurrences");
    recurrences_path
        // Insert an Habit into databases
        .and(warp::post())
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::body::json())
        .and_then(recurrency_handler::create_recurrency_handler)
        .or(
            // Get habits from database (for a given user)
            recurrences_path
                .and(warp::get())
                .and(warp::path::param::<uuid::Uuid>())
                .and_then(recurrency_handler::get_recurrences_handler),
        )
        .or(
            // Update an habit from database
            recurrences_path
                .and(warp::patch())
                .and(warp::path::param::<uuid::Uuid>())
                .and(warp::body::json())
                .and_then(recurrency_handler::update_recurrence_handler),
        )
        .or(
            // Delete an habit from database
            recurrences_path
                .and(warp::delete())
                .and(warp::path::param::<uuid::Uuid>())
                .and_then(recurrency_handler::delete_recurrence_handler),
        )
        .boxed()
}
