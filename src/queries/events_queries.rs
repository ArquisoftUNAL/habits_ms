use crate::{
    db::DBManager,
    error::Error,
    models::api::{events_api_models::*, habit_api_models::*},
    models::database::{Habit, HabitRecurrence, RecDataEnum},
    queries::recurrences_queries,
    schema::*,
    utils::{queries::join_habit_with_recurrences, DEFAULT_QUERY_LIMIT, MAX_QUERY_LIMIT},
};

use diesel::prelude::*;

use uuid::Uuid;
use warp::filters::sse::Event;

impl DBManager {
    // Get next events for user based on recurrences
    pub fn get_next_events_with_habits(
        &self,
        habits: Vec<Habit>,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
    ) -> Result<Vec<EventWithHabit>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let vec = Vec::new();

        Ok(vec)
    }
}
