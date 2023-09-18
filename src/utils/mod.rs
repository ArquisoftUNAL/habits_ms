pub mod queries;
pub mod time;

use crate::db::{DBManager, PostgresPool};
use warp::{Filter, Rejection};

// Define constants
pub const MAX_QUERY_LIMIT: i64 = 100;
pub const DEFAULT_QUERY_LIMIT: i64 = 100;

pub fn with_db_manager(
    pool: PostgresPool,
) -> impl Filter<Extract = (DBManager,), Error = Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(
            |pool: PostgresPool| async move { Ok::<DBManager, Rejection>(DBManager::new(pool)) },
        )
}
