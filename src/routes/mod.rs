pub mod v1;

use crate::{db::PostgresPool, error::handle_rejection};
use std::convert::Infallible;
use warp::Filter;

pub fn get_routes(
    pool_write: Option<PostgresPool>,
    pool_read: Option<PostgresPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    let api = warp::path("api");

    // Expand in case there are more api versions
    api.and(v1::get_routes(pool_write.clone(), pool_read.clone()))
        .recover(handle_rejection)
}
