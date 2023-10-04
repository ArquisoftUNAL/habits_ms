pub mod queries;
pub mod time;

use crate::db::{DBManager, PostgresPool};
use crate::models::api::{AuthData, AuthRole};
use warp::{Filter, Rejection};

// Define constants
pub const MAX_QUERY_LIMIT: i64 = 100;
pub const DEFAULT_QUERY_LIMIT: i64 = 100;
pub const MAX_DAYS_OFFSET: i64 = 1; // Grace period a user will be given to mark a habit as completed
pub const HABIT_CREATION_DATE_AS_REFERENCE: bool = true; // Habit's creation date represents the start of its own recurrences

pub fn with_db_manager(
    pool: PostgresPool,
) -> impl Filter<Extract = (DBManager,), Error = Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(
            |pool: PostgresPool| async move { Ok::<DBManager, Rejection>(DBManager::new(pool)) },
        )
}

pub fn with_authenticator() -> impl Filter<Extract = (AuthData,), Error = Rejection> + Clone {
    // Get authentication data
    warp::any()
        .and(warp::header::<String>("user_id"))
        .map(|identification: String| -> AuthData {
            AuthData {
                requester_id: identification,
                role: AuthRole::User,
            }
        })
        .or(warp::any().map(|| AuthData {
            requester_id: "".to_string(),
            role: AuthRole::Guest,
        }))
        .unify()
        .and_then(|auth| async move { Ok::<AuthData, Rejection>(auth) })
}
