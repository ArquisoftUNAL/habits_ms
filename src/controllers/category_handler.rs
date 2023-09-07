use crate::{
    db::POSTGRES_POOL as pool,
    models::{
        api::{category_api_models::*, *},
        database::Category,
    },
    queries::categories_queries,
    schema::*,
};
use diesel::prelude::*;

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;

// GET Route
pub async fn get_categories_handler() -> Result<impl Reply, Rejection> {
    let result = categories_queries::get_all_categories().await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error getting categories: {}", error),
        };
        return Ok(json(&response));
    }

    let response = CategoryMultipleQueryResponse {
        status: 200,
        categories: result.unwrap(),
    };

    Ok(json(&response))
}

// GET Route
pub async fn get_category_by_id_handler(id: Uuid) -> Result<impl Reply, Rejection> {
    let result = categories_queries::get_category_by_id(id).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error getting category: {}", error),
        };
        return Ok(json(&response));
    }

    let response = CategorySingleQueryResponse {
        status: 200,
        category: result.unwrap(),
    };

    Ok(json(&response))
}

// POST Route
pub async fn create_category_handler(data: CategoryCreateSchema) -> Result<impl Reply, Rejection> {
    let result = categories_queries::add_category(data).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error creating category: {}", error),
        };
        return Ok(json(&response));
    }

    let response = GeneralResponse {
        status: 200,
        message: "Successfully added category".to_string(),
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_category_handler(id: Uuid) -> Result<impl Reply, Rejection> {
    let result = categories_queries::delete_category(id).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 203,
            message: format!("Error deleting category {}", error),
        };

        return Ok(json(&response));
    }

    let response = GeneralResponse {
        status: 200,
        message: "Successfully deleted category".to_string(),
    };

    Ok(json(&response))
}

// UPDATE Route
pub async fn update_category_handler(
    id: Uuid,
    data: CategoryCreateSchema,
) -> Result<impl Reply, Rejection> {
    let result = categories_queries::update_category(id, data).await;

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 200,
            message: format!("Error updating category {}", error),
        };

        return Ok(json(&response));
    }

    let response = GeneralResponse {
        status: 200,
        message: "Successfully updated category".to_string(),
    };

    Ok(json(&response))
}
