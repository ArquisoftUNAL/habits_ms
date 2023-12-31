pub mod category_route;
pub mod events_route;
pub mod habit_data_route;
pub mod habits_route;
pub mod ownership_route;

use crate::db::PostgresPool;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(
    pool_write: Option<PostgresPool>,
    pool_read: Option<PostgresPool>,
) -> BoxedFilter<(impl Reply,)> {
    let v1 = warp::path("v1");

    v1.and(habits_route::get_routes(
        pool_write.clone(),
        pool_read.clone(),
    ))
    .or(v1.and(habit_data_route::get_routes(
        pool_write.clone(),
        pool_read.clone(),
    )))
    .or(v1.and(category_route::get_routes(
        pool_write.clone(),
        pool_read.clone(),
    )))
    .or(v1.and(events_route::get_routes(
        pool_write.clone(),
        pool_read.clone(),
    )))
    .or(v1.and(ownership_route::get_routes(
        pool_write.clone(),
        pool_read.clone(),
    )))
    .boxed()
}
