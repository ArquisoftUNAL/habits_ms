use crate::models::database::Habit;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use warp::{
    http::{Response, StatusCode},
    reject,
    reply::json,
    Rejection, Reply,
};

#[derive(serde::Serialize)]
pub struct Data {
    integer: i32,
    text: String,
}

pub async fn create_habit_handler(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<impl Reply, Rejection> {
    let data = Data {
        integer: 1,
        text: "hello".to_string(),
    };
    Ok(json(&data))
}
