use crate::{
    models::api::{GeneralResponse, HabitCreateRequest},
    models::database::Habit,
    schema::*,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use warp::{reply::json, Rejection, Reply};

pub async fn insert_recurrency(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<impl Reply, Rejection> {
    let response = GeneralResponse {
        status: 200,
        message: "Gotten habits successfully".to_string(),
    };
    Ok(json(&response))
}

pub async fn get_habit_recurrencies(
    pool: Pool<ConnectionManager<PgConnection>>,
    habitId: uuid::Uuid,
) -> Result<impl Reply, Rejection> {
    let response = GeneralResponse {
        status: 200,
        message: "Gotten habits successfully".to_string(),
    };
    Ok(json(&response))
}
