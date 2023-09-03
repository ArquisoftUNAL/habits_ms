pub mod habits_route;

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn get_routes(pool: Pool<ConnectionManager<PgConnection>>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("v1" / ..)
        .and(habits_route::get_routes(pool))
        .boxed()
}
