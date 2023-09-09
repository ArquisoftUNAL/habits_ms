use crate::{
    db::DBManager,
    models::api::{category_api_models::*, *},
};

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;
use validator::Validate;

// GET Route
pub async fn get_categories_handler(manager: DBManager) -> Result<impl Reply, Rejection> {
    let result = manager.get_all_categories();

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
) -> Result<impl Reply, Rejection> {
    let result = manager.get_category_by_id(id);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error getting category: {}", error),
        };
        return Ok(json(&response));
    }

    let response = CategorySingleQueryResponse {
        message: format!("Successfully retrieved category"),
        category: result.unwrap(),
    };

    Ok(json(&response))
}

// POST Route
pub async fn create_category_handler(
    manager: DBManager,
    data: CategoryCreateSchema,
) -> Result<impl Reply, Rejection> {
    // Validate input
    let validation_result = CategoryCreateSchema::validate(&data);
    if validation_result.is_err() {
        let error = validation_result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error validating category: {}", error),
        };
        return Ok(json(&response));
    }

    let result = manager.add_category(data);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error creating category: {}", error),
        };
        return Ok(json(&response));
    }

    let response = GeneralResponse {
        message: "Successfully added category".to_string(),
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
        let response = GeneralResponse {
            message: format!("Error deleting category {}", error),
        };

        return Ok(json(&response));
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
    data: CategoryCreateSchema,
) -> Result<impl Reply, Rejection> {
    let result = manager.update_category(id, data);

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            message: format!("Error updating category {}", error),
        };

        return Ok(json(&response));
    }

    let response = GeneralResponse {
        message: "Successfully updated category".to_string(),
    };

    Ok(json(&response))
}
