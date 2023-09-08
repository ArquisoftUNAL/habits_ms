use crate::handlers::category_handler;

use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use uuid::Uuid;

pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    let categories_path = warp::path("categories");
    categories_path
        // Insert an Habit into databases
        .and(warp::any())
        .and(warp::body::json())
        .and_then(category_handler::create_category_handler)
        .or(
            // Get all categories from database
            categories_path
                .and(warp::get())
                //.and(warp::path::param::<String>())
                .and_then(category_handler::get_categories_handler)
                .and(warp::path::end()),
        )
        .or(
            // Get a category from database by ID
            categories_path
                .and(warp::get())
                .and(warp::path::param::<Uuid>())
                .and_then(category_handler::get_category_by_id_handler),
        )
        .or(
            // Update category from database
            categories_path
                .and(warp::patch())
                .and(warp::path::param::<Uuid>())
                .and(warp::body::json())
                .and_then(category_handler::update_category_handler),
        )
        .or(
            // Delete category from database
            categories_path
                .and(warp::delete())
                .and(warp::path::param::<Uuid>())
                .and_then(category_handler::delete_category_handler),
        )
        .boxed()
}
