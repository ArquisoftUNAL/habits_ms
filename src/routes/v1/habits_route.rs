use crate::{controllers::habits_handler, db::POSTGRES_POOL as pool, utils::with_db};

use warp::filters::BoxedFilter;
use warp::path;
use warp::Filter;
use warp::Reply;

pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    let habits_path = warp::path("habits");
    habits_path
        .and(warp::post())
        .and(warp::body::json())
        .and(path::end())
        .and_then(habits_handler::create_habit_handler)
        .or(habits_path
            .and(warp::get())
            .and(path::end())
            .and_then(habits_handler::get_habits_handler))
        .boxed()
}
