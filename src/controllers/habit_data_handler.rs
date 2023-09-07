use crate::{
    db::POSTGRES_POOL as pool,
    models::{
        api::{data_api_models::*, *},
        database::HabitDataCollected,
    },
    schema::*,
};
use diesel::prelude::*;

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;

// CREATE Route
pub async fn create_habit_data_handler(
    recurrency_id: Uuid,
    body: HabitDataRequest,
) -> Result<impl Reply, Rejection> {
    // Create model from request body
    let data = HabitDataCollected {
        hab_dat_id: uuid::Uuid::new_v4(),
        hab_dat_amount: body.amount,
        hab_dat_collected_at: chrono::Local::now().naive_local(),
        hab_rec_id: recurrency_id,
    };
    // Add habit to database
    let result = diesel::insert_into(habit_data_collected::table)
        .values(&data)
        .execute(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error creating habit: {}", error),
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

// READ Route
pub async fn get_habit_data_handler(recurrency_id: Uuid) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let result = habit_data_collected::table
        .select(HabitDataCollected::as_select())
        .filter(habit_data_collected::hab_rec_id.eq(recurrency_id))
        .load::<HabitDataCollected>(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error getting habits: {}", error),
        };
        return Ok(json(&response));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    if result.len() == 0 {
        let response = GeneralResponse {
            status: 404,
            message: "User / Habits not found".to_string(),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = HabitDataMultipleQuery {
        status: 200,
        habits: result,
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_habit_data_handler(habit_data_id: Uuid) -> Result<impl Reply, Rejection> {
    // Delete habit from database
    let result = diesel::delete(
        habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(habit_data_id)),
    )
    .execute(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error deleting habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        status: 200,
        message: "Habit deleted successfully".to_string(),
    };
    Ok(json(&response))
}

// UPDATE (PATCH) Route
pub async fn update_habit_data_handler(
    habit_data_id: Uuid,
    body: HabitDataRequest,
) -> Result<impl Reply, Rejection> {
    let result = diesel::update(
        habit_data_collected::table.filter(habit_data_collected::hab_dat_id.eq(habit_data_id)),
    )
    .set((
        habit_data_collected::hab_dat_amount.eq(body.amount),
        habit_data_collected::hab_dat_collected_at.eq(chrono::Local::now().naive_local()),
        habit_data_collected::hab_rec_id.eq(body.recurrency_id),
    ))
    .execute(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error updating habit: {}", error),
        };
        return Ok(json(&response));
    }

    // Return response
    let response = GeneralResponse {
        status: 200,
        message: "Habit updated successfully".to_string(),
    };

    Ok(json(&response))
}
