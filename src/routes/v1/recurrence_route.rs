use crate::{
    db::PostgresPool,
    handlers::recurrence_handler,
    models::api::{DataIncludeParams, RangeParams},
    utils::with_db_manager,
};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let base_recurrence_route = warp::path("recurrences");

    let create_recurrence = base_recurrence_route
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(warp::body::json())
        .and_then(recurrence_handler::create_recurrence_handler);

    let update_recurrence = base_recurrence_route
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::body::json())
        .and_then(recurrence_handler::update_recurrence_handler);

    let delete_recurrence = base_recurrence_route
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<uuid::Uuid>())
        .and_then(recurrence_handler::delete_recurrence_handler);

    // Getting habits by user id
    let base_get_recurrence_route = base_recurrence_route
        .and(warp::get())
        .and(warp::path::param::<uuid::Uuid>())
        .and(warp::query::<RangeParams>());

    let get_recurrence_by_id = base_get_recurrence_route
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            include_recurrences: Some(true),
            ..Default::default()
        }))
        .and(warp::path::end())
        .and_then(recurrence_handler::get_recurrence_by_id_handler);

    let get_recurrence_by_id_data = base_get_recurrence_route
        .and(warp::path("data"))
        .and(with_db_manager(pool.clone()))
        .and(warp::any().map(move || DataIncludeParams {
            include_data: Some(true),
            ..Default::default()
        }))
        .and_then(recurrence_handler::get_recurrence_by_id_handler);

    create_recurrence
        .or(update_recurrence)
        .or(delete_recurrence)
        .or(get_recurrence_by_id)
        .or(get_recurrence_by_id_data)
        .boxed()
}
