use crate::{
    db::POSTGRES_POOL as pool,
    models::api::{GeneralResponse, HabitCreateRequest},
    models::database::Habit,
    schema::*,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use warp::{reply::json, Rejection, Reply};

pub async fn create_habit_handler(body: HabitCreateRequest) -> Result<impl Reply, Rejection> {
    // Create model from request body
    let habit = Habit {
        id: uuid::Uuid::new_v4(),
        name: body.name,
        description: body.description,
        createdAt: chrono::Local::now().naive_local(),
        updatedAt: chrono::Local::now().naive_local(),
        isFavorite: body.isFavorite,
        kind: body.kind,
        userId: body.userId,
    };
    // Add habit to database
    let result = diesel::insert_into(habit::table)
        .values(&habit)
        .execute(&mut pool.get().unwrap());

    if result.is_err() {
        let response = GeneralResponse {
            status: 400,
            message: "Error creating habit".to_string(),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        status: 201,
        message: "Habit created successfully".to_string(),
    };
    Ok(json(&response))
}

pub async fn get_habits_handler() -> Result<impl Reply, Rejection> {
    let response = GeneralResponse {
        status: 200,
        message: "Gotten habits successfully".to_string(),
    };
    Ok(json(&response))
}
