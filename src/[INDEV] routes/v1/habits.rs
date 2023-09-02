use crate::controllers::habits;
use std::convert::Infallible;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use warp::Filter;

pub fn get_routes(// pool: Pool<ConnectionManager<PgConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("habits" / ..)
        .and(warp::get())
        // .and(warp::any().map(move || pool.clone()))
        .and_then(habits::create_habit_handler)
}
