pub mod v1;

use warp::filters::BoxedFilter;
use warp::Filter;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn get_routes(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / ..).and(v1::get_routes(pool))
}
