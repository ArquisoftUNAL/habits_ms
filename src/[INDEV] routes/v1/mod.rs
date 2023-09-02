mod habits;

use warp::Filter;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use std::convert::Infallible;

pub fn get_routes(// pool: Pool<ConnectionManager<PgConnection>>,
) -> (impl warp::Reply, impl warp::Reply) {
    // let v1 = warp::path!("v1").and(habits::setup(pool));
    // return v1;
    warp::path!("v1" / ..).and(habits::get_routes());
}
