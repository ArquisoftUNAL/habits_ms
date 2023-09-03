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

pub async fn add_habit_data() -> Result<impl Reply, Rejection> {
    let response = GeneralResponse {
        status: 200,
        message: "Gotten habits successfully".to_string(),
    };
    Ok(json(&response))
}

pub async fn get_habit_data(habitId: uuid::Uuid) -> Result<impl Reply, Rejection> {
    let response = GeneralResponse {
        status: 200,
        message: "Gotten habits successfully".to_string(),
    };
    Ok(json(&response))
}
