use crate::{
    db::PostgresPool,
    handlers::category_handler,
    models::api::{DataIncludeParams, RangeParams},
    utils::{with_authenticator, with_db_manager},
};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use uuid::Uuid;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let base_category_route = warp::path("categories");
    let create_category = base_category_route
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::body::json())
        .and_then(category_handler::create_category_handler);

    let update_category = base_category_route
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::path::param::<Uuid>())
        .and(warp::body::json())
        .and_then(category_handler::update_category_handler);

    let delete_category = base_category_route
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(with_authenticator())
        .and(warp::path::param::<Uuid>())
        .and_then(category_handler::delete_category_handler);

    let get_categories = base_category_route
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::query::<RangeParams>())
        .and(warp::path::end())
        .and_then(category_handler::get_categories_handler);

    let get_category_by_id = base_category_route
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and(warp::query::<RangeParams>())
        .and(warp::any().map(move || DataIncludeParams {
            ..Default::default()
        }))
        .and_then(category_handler::get_category_by_id_handler);

    create_category
        .or(get_categories)
        .or(get_category_by_id)
        .or(update_category)
        .or(delete_category)
        .boxed()
}
