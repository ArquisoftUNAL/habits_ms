pub mod v1;

use crate::error::handle_rejection;
use std::convert::Infallible;
use warp::Filter;

pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    let api = warp::path("api");

    // Expand in case there are more api versions
    api.and(v1::get_routes()).recover(handle_rejection)
}
