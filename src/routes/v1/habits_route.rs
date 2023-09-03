use crate::controllers::habits_handler;

use warp::filters::BoxedFilter;
use warp::path;
use warp::Filter;
use warp::Reply;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn get_routes(pool: Pool<ConnectionManager<PgConnection>>) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("habits"))
        .and(warp::post())
        .and(warp::any().map(move || pool.clone()))
        .and(warp::body::json())
        .and(path::end())
        .and_then(habits_handler::create_habit_handler)
        .boxed()
}
