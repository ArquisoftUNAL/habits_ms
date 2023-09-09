use crate::{db::PostgresPool, handlers::category_handler, utils::with_db_manager};

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use uuid::Uuid;

pub fn get_routes(pool: PostgresPool) -> BoxedFilter<(impl Reply,)> {
    let create_category = warp::path("categories")
        .and(warp::post())
        .and(with_db_manager(pool.clone()))
        .and(warp::body::json())
        .and_then(category_handler::create_category_handler);

    let get_categories = warp::path("categories")
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and_then(category_handler::get_categories_handler);

    let get_category_by_id = warp::path("categories")
        .and(warp::get())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and_then(category_handler::get_category_by_id_handler);

    let update_category = warp::path("categories")
        .and(warp::patch())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::json())
        .and_then(category_handler::update_category_handler);

    let delete_category = warp::path("categories")
        .and(warp::delete())
        .and(with_db_manager(pool.clone()))
        .and(warp::path::param::<Uuid>())
        .and_then(category_handler::delete_category_handler);

    create_category
        .or(get_categories)
        .or(get_category_by_id)
        .or(update_category)
        .or(delete_category)
        .boxed()
}
