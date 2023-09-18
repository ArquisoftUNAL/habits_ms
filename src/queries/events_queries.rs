use std::collections::HashMap;

use crate::{
    db::DBManager,
    error::Error,
    models::api::events_api_models::*,
    models::database::{Habit, HabitRecurrence},
    schema::*,
    utils::{time::DateRange, DEFAULT_QUERY_LIMIT},
};
use diesel::prelude::*;

impl DBManager {
    // Get next events for user based on recurrences
    pub fn get_next_events_with_habits(
        &self,
        id: String,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
        events_limit: Option<i64>,
    ) -> Result<Vec<EventWithHabit>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let events_limit = events_limit.unwrap_or(DEFAULT_QUERY_LIMIT);

        let query = habit::table
            .inner_join(habit_recurrence::table)
            .filter(habit::usr_id.eq(id))
            .select((Habit::as_select(), HabitRecurrence::as_select()))
            .order_by(habit_recurrence::hab_rec_freq_data.asc())
            .load::<(Habit, HabitRecurrence)>(&mut conn.unwrap());

        if query.is_err() {
            return Err(Error::QueryError(query.err().unwrap()));
        }

        let query = query.unwrap();

        let start_date = start_date.unwrap_or(chrono::Local::now().naive_local().date());

        // By default, end date is 7 days from start date (so a week)
        let end_date = end_date.unwrap_or(start_date + chrono::Duration::days(7));

        let mut vec = Vec::new();

        for (habit, recurrence) in query {
            let data_range = DateRange::new(
                start_date,
                end_date,
                recurrence.hab_rec_freq_type,
                recurrence.hab_rec_freq_data,
            );

            for date_ocurrence in data_range {
                let event = EventWithHabit {
                    date: date_ocurrence,
                    habit: habit.clone(),
                    recurrence: recurrence.clone(),
                };

                vec.push(event);
            }

            // Limit array length to 100
            if vec.len() >= events_limit as usize {
                break;
            }
        }
        Ok(vec)
    }

    // Get next events for user based on recurrences
    pub fn get_next_events_counts(
        &self,
        id: String,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
    ) -> Result<Vec<EventWithCount>, Error> {
        let conn = self.connection.get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        let query = habit::table
            .inner_join(habit_recurrence::table)
            .filter(habit::usr_id.eq(id))
            .select((Habit::as_select(), HabitRecurrence::as_select()))
            .order_by(habit_recurrence::hab_rec_freq_data.asc())
            .load::<(Habit, HabitRecurrence)>(&mut conn.unwrap());

        if query.is_err() {
            return Err(Error::QueryError(query.err().unwrap()));
        }

        let query = query.unwrap();

        let start_date = start_date.unwrap_or(chrono::Local::now().naive_local().date());

        // By default, end date is 7 days from start date (so a week)
        let end_date = end_date.unwrap_or(start_date + chrono::Duration::days(7));

        let mut vec = Vec::new();
        let mut dates_map = HashMap::new();

        for (_, recurrence) in query {
            let data_range = DateRange::new(
                start_date,
                end_date,
                recurrence.hab_rec_freq_type,
                recurrence.hab_rec_freq_data,
            );

            for date_ocurrence in data_range {
                if !dates_map.contains_key(&date_ocurrence) {
                    dates_map.insert(date_ocurrence, 1);
                } else {
                    let count = dates_map.get_mut(&date_ocurrence).unwrap();
                    *count += 1;
                }
            }
        }

        for (date, count) in dates_map {
            let event = EventWithCount { date, count };

            vec.push(event);
        }
        Ok(vec)
    }
}
