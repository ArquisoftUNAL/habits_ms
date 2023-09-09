pub mod v1;

use crate::{
    db::PostgresPool, error::handle_rejection, handlers::habit_handler, utils::with_db_manager,
};
use std::convert::Infallible;
use warp::{
    reject::{Reject, Rejection},
    Filter,
};

pub fn get_routes(
    pool: PostgresPool,
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    // warp::path!("api" / "v1" / "habits" / "user" / String)
    //     .and(warp::get())
    //     .and(with_db_manager(pool.clone()))
    //     .and_then(habit_handler::get_habits_handler)
    //     .recover(handle_rejection)

    let api = warp::path("api");

    // Expand in case there are more api versions
    api.and(v1::get_routes(pool.clone()))
        .recover(handle_rejection)
}
