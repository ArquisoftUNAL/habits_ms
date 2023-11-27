use crate::{
    db::PostgresPool,
    handlers::ownership_handler,
    utils::{with_authenticator, with_db_manager},
};

use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(
    pool_write: Option<PostgresPool>,
    pool_read: Option<PostgresPool>,
) -> BoxedFilter<(impl Reply,)> {
    let check_habit_ownership = warp::path("ownership")
        .and(warp::path("habit"))
        .and(warp::get())
        .and(warp::path::param::<Uuid>())
        .and(with_db_manager(pool_write.clone(), pool_read.clone()))
        .and(with_authenticator())
        .and_then(ownership_handler::check_ownership_handler);

    let check_data_ownership = warp::path("ownership")
        .and(warp::path("data"))
        .and(warp::get())
        .and(warp::path::param::<Uuid>())
        .and(with_db_manager(pool_write.clone(), pool_read.clone()))
        .and(with_authenticator())
        .and_then(ownership_handler::check_data_ownership_handler);

    check_habit_ownership.or(check_data_ownership).boxed()
}
