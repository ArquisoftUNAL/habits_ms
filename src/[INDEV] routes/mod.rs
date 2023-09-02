mod v1;

use warp::Filter;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use std::convert::Infallible;

// pub fn setup(
//     pool: Pool<ConnectionManager<PgConnection>>,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     // let v1 = warp::path("v1").and(v1::setup(pool));
//     let api = warp::path("api").and(v1::setup(pool));
//     return warp::path("api");
// }

pub fn get_routes(// pool: Pool<ConnectionManager<PgConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("v1").and(v1::get_routes())
}
