use crate::{
    db::DBManager,
    error::Error,
    models::api::{category_api_models::*, *},
};

use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Rejection, Reply,
};

use uuid::Uuid;
use validator::Validate;

// GET Route
pub async fn get_categories_handler(
    manager: DBManager,
    params: RangeParams,
) -> Result<impl Reply, Rejection> {
    let result = manager.get_all_categories(params.categories_page, params.categories_per_page);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error getting categories: {}", error),
        };
        return Ok(json(&response));
    }

    let response = CategoryMultipleQueryResponse {
        message: format!("Successfully retrieved categories"),
        categories: result.unwrap(),
    };

    Ok(json(&response))
}

// GET Route
pub async fn get_category_by_id_handler(
    manager: DBManager,
    id: Uuid,
    _params: RangeParams,
    _data_params: DataIncludeParams,
) -> Result<impl Reply, Rejection> {
    let result = manager.get_category_by_id(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let response = CategorySingleQueryResponse {
        message: format!("Successfully retrieved category"),
        category: result.unwrap(),
    };

    Ok(with_status(json(&response), StatusCode::OK))
}

// POST Route
pub async fn create_category_handler(
    manager: DBManager,
    data: CategoryCreateSchema,
) -> Result<impl Reply, Rejection> {
    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    let result = manager.add_category(data);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let response = CategoryCreateResponse {
        message: "Successfully added category".to_string(),
        id: result.unwrap(),
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_category_handler(
    manager: DBManager,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    let result = manager.delete_category(id);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let response = GeneralResponse {
        message: "Successfully deleted category".to_string(),
    };

    Ok(json(&response))
}

// UPDATE Route
pub async fn update_category_handler(
    manager: DBManager,
    id: Uuid,
    data: CategoryUpdateSchema,
) -> Result<impl Reply, Rejection> {
    // Validate input
    let validation_result = data.validate();

    if validation_result.is_err() {
        return Err(warp::reject::custom(Error::ValidationError(
            validation_result.err().unwrap(),
        )));
    }

    let result = manager.update_category(id, data);

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(warp::reject::custom(error));
    }

    let response = GeneralResponse {
        message: "Successfully updated category".to_string(),
    };

    Ok(json(&response))
}
