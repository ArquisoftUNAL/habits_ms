use crate::{db::DBManager, error::Error, models::api::*};

use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Rejection, Reply,
};

use uuid::Uuid;

// GET Route
pub async fn check_ownership_handler(
    id: Uuid,
    manager: DBManager,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check a user is logged in / provided the action
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    if matches!(authentication.role, AuthRole::User) {
        let result = manager.is_habit_accessible_by_user(authentication.requester_id, id);

        if result.is_err() {
            let error = result.err().unwrap();
            return Err(warp::reject::custom(error));
        }

        if !result.unwrap() {
            return Err(warp::reject::custom(Error::AuthorizationError(
                "User has not access to this habit".to_string(),
            )));
        }
    }

    Ok(with_status(json(&()), StatusCode::OK))
}

pub async fn check_data_ownership_handler(
    id: Uuid,
    manager: DBManager,
    authentication: AuthData,
) -> Result<impl Reply, Rejection> {
    // Check a user is logged in / provided the action
    if matches!(authentication.role, AuthRole::Guest) {
        return Err(warp::reject::custom(Error::AuthorizationError(
            "Missing user id in request header (user_id)".to_string(),
        )));
    }

    if matches!(authentication.role, AuthRole::User) {
        let result = manager.is_habitdata_accessible_by_user(authentication.requester_id, id);

        if result.is_err() {
            let error = result.err().unwrap();
            return Err(warp::reject::custom(error));
        }

        if !result.unwrap() {
            return Err(warp::reject::custom(Error::AuthorizationError(
                "User has not access to this habit data".to_string(),
            )));
        }
    }

    Ok(with_status(json(&()), StatusCode::OK))
}
