pub mod category_route;
pub mod habit_data_route;
pub mod habits_route;
pub mod recurrency_route;

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    let v1 = warp::path("v1");
    v1.and(habits_route::get_routes())
        .or(v1.and(recurrency_route::get_routes()))
        .or(v1.and(habit_data_route::get_routes()))
        .or(v1.and(category_route::get_routes()))
        .boxed()
}
