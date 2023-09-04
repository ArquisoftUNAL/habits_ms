pub mod v1;

use warp::Filter;

pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api = warp::path("api");

    // Expand in case there are more api versions
    api.and(v1::get_routes())
}
