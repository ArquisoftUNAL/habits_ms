use crate::{
    db::POSTGRES_POOL as pool,
    models::{
        api::{recurrency_api_models::*, *},
        database::HabitRecurrency,
    },
    schema::*,
};
use diesel::prelude::*;

use warp::{reply::json, Rejection, Reply};

use uuid::Uuid;

// CREATE Route
pub async fn create_recurrency_handler(
    habit_id: Uuid,
    body: RecurrencyCreateRequest,
) -> Result<impl Reply, Rejection> {
    // Create model from request body
    let recurrency = HabitRecurrency {
        hab_rec_id: uuid::Uuid::new_v4(),
        hab_rec_freq_type: body.frequency_type,
        hab_rec_freq_data: body.frequency_data,
        hab_id: habit_id,
    };
    // Add habit to database
    let result = diesel::insert_into(habit_recurrency::table)
        .values(&recurrency)
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
pub async fn get_recurrences_handler(habit_id: Uuid) -> Result<impl Reply, Rejection> {
    // Get habits from database
    let result = habit_recurrency::table
        .select(HabitRecurrency::as_select())
        .filter(habit_recurrency::hab_id.eq(habit_id))
        .load::<HabitRecurrency>(&mut pool.get().unwrap());

    if result.is_err() {
        let error = result.err().unwrap();
        let response = GeneralResponse {
            status: 400,
            message: format!("Error getting habit's recurrences: {}", error),
        };
        return Ok(json(&response));
    }

    // Check if user was not found (actually if no habits are related to it)
    let result = result.unwrap();

    // Return response
    let response = RecurrencesMultipleQuery {
        status: 200,
        habits: result,
    };

    Ok(json(&response))
}

// DELETE Route
pub async fn delete_recurrence_handler(recurrence_id: Uuid) -> Result<impl Reply, Rejection> {
    // Delete habit from database
    let result = diesel::delete(
        habit_recurrency::table.filter(habit_recurrency::hab_rec_id.eq(recurrence_id)),
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
pub async fn update_recurrence_handler(
    recurrence_id: Uuid,
    body: RecurrencyCreateRequest,
) -> Result<impl Reply, Rejection> {
    let result = diesel::update(
        habit_recurrency::table.filter(habit_recurrency::hab_rec_id.eq(recurrence_id)),
    )
    .set((
        habit_recurrency::hab_id.eq(body.habit_id),
        habit_recurrency::hab_rec_freq_type.eq(body.frequency_type),
        habit_recurrency::hab_rec_freq_data.eq(body.frequency_data),
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
