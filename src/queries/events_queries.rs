use crate::{
    db::DBManager,
    error::Error,
    models::{api::events_api_models::*, database::Habit},
    schema::*,
    utils::{time::DateRange, DEFAULT_QUERY_LIMIT, HABIT_CREATION_DATE_AS_REFERENCE},
};
use diesel::prelude::*;
use uuid::Uuid;

impl DBManager {
    // Get next period closure events from habit
    pub fn get_habit_next_events(
        &self,
        id: Uuid,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
        events_limit: Option<i64>,
    ) -> Result<Vec<Event>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let events_limit = events_limit.unwrap_or(DEFAULT_QUERY_LIMIT);

        let query = habit::table
            .select(Habit::as_select())
            .find(id)
            .first(&mut conn.unwrap());

        if query.is_err() {
            return Err(Error::QueryError(query.err().unwrap()));
        }

        let habit: Habit = query.unwrap();

        let start_date = start_date.unwrap_or(chrono::Local::now().naive_local().date());

        // By default, end date is 7 days from start date (so a week)
        let end_date = end_date.unwrap_or(start_date + chrono::Duration::days(7));

        let mut vec = Vec::new();

        let data_range = DateRange::new(
            end_date,
            habit.hab_freq_type,
            Some(start_date),
            Some(match HABIT_CREATION_DATE_AS_REFERENCE {
                true => habit.hab_created_at.date(),
                false => start_date,
            }),
        );

        for date_ocurrence in data_range {
            let event = Event {
                date: date_ocurrence,
            };

            vec.push(event);

            // Limit array length to 100
            if vec.len() >= events_limit as usize {
                break;
            }
        }

        // Order by date (most recent first)
        vec.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(vec)
    }
}
