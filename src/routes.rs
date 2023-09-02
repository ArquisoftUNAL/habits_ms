use warp::Filter;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::controllers::habits;

pub fn get_routes(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Habits Routes
    // TODO: Refactorize and modularize routes definition

    let habits = warp::path!("habits")
        .and(warp::get())
        .and(warp::any().map(move || pool.clone()))
        .and_then(habits::create_habit_handler);

    let api = warp::path!("api" / "v1" / ..).and(habits);

    return api;
}
