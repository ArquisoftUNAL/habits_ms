pub mod category_route;
pub mod habit_data_route;
pub mod habits_route;
pub mod recurrence_route;

use crate::db::PostgresPool;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let v1 = warp::path("v1");

    v1.and(habits_route::get_routes(pool.clone()))
        .or(v1.and(recurrence_route::get_routes(pool.clone())))
        .or(v1.and(habit_data_route::get_routes(pool.clone())))
        .or(v1.and(category_route::get_routes(pool.clone())))
        .boxed()
}
