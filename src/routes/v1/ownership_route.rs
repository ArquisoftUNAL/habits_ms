use crate::{
    db::PostgresPool,
    handlers::ownership_handler,
    utils::{with_authenticator, with_db_manager},
};

use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let check_ownership = warp::path("ownership")
        .and(warp::get())
        .and(warp::path::param::<Uuid>())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and_then(ownership_handler::check_ownership_handler);

    check_ownership.boxed()
}
