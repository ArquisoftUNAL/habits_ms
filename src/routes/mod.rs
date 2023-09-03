pub mod v1;

use warp::filters::BoxedFilter;
use warp::Filter;

pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / ..).and(v1::get_routes())
}
