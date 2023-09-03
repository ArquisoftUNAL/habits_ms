pub mod habits_route;

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    warp::path!("v1" / ..)
        .and(habits_route::get_routes())
        .boxed()
}
