use crate::{
    db::DBManager,
    error::Error,
    models::{
        api::events_api_models::*,
        database::{Habit, HabitDataCollected},
    },
    schema::*,
    utils::{
        time::{DateRange, MAXIMUM_DATE, MINIMUM_DATE},
        DEFAULT_QUERY_LIMIT, HABIT_CREATION_DATE_AS_REFERENCE,
    },
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
        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
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

    // Summarize habit data between two dates
    pub fn get_habitdata_as_calendar(
        &self,
        user_id: Option<String>,
        habit_id: Option<Uuid>,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
    ) -> Result<Vec<CalendarEvent>, Error> {
        let conn = self.get_read_connection();

        if conn.is_err() {
            return Err(conn.err().unwrap());
        }

        let mut query = habit_data_collected::table
            .inner_join(habit::table)
            .select((HabitDataCollected::as_select(), Habit::as_select()))
            .filter(
                habit_data_collected::hab_dat_collected_at
                    .ge(start_date.unwrap_or(MINIMUM_DATE.unwrap())),
            )
            .filter(
                habit_data_collected::hab_dat_collected_at
                    .le(end_date.unwrap_or(MAXIMUM_DATE.unwrap())),
            )
            .into_boxed();

        if habit_id.is_some() {
            query = query.filter(habit::hab_id.eq(habit_id.unwrap()));
        } else if user_id.is_some() {
            query = query.filter(habit::usr_id.eq(user_id.unwrap()));
        } else {
            return Err(Error::BadRequest("Missing habit_id or user_id".to_string()));
        }

        query = query.order(habit_data_collected::hab_dat_collected_at.asc());

        // Execute query
        let data = query.load::<(HabitDataCollected, Habit)>(&mut conn.unwrap());

        if data.is_err() {
            return Err(Error::QueryError(data.err().unwrap()));
        }

        let data = data.unwrap();

        // Group data by date
        let mut data_by_date: Vec<CalendarEvent> = Vec::new();

        for (habit_data, habit) in data {
            let mut found = false;

            for data_by_date_item in &mut data_by_date {
                if data_by_date_item.date == habit_data.hab_dat_collected_at {
                    data_by_date_item.data += match habit.hab_is_yn {
                        true => bigdecimal::BigDecimal::from(1),
                        false => habit_data.hab_dat_amount.clone(),
                    };

                    found = true;
                    break;
                }
            }

            if !found {
                let calendar_event = CalendarEvent {
                    date: habit_data.hab_dat_collected_at,
                    data: match habit.hab_is_yn {
                        true => bigdecimal::BigDecimal::from(1),
                        false => habit_data.hab_dat_amount,
                    },
                    relative_frequency: bigdecimal::BigDecimal::from(0),
                };

                data_by_date.push(calendar_event);
            }
        }

        // Find relative frequency
        let mut total_data = bigdecimal::BigDecimal::from(0);

        for data_by_date_item in &data_by_date {
            total_data += data_by_date_item.data.clone();
        }

        for data_by_date_item in &mut data_by_date {
            data_by_date_item.relative_frequency =
                data_by_date_item.data.clone() / total_data.clone();
        }

        Ok(data_by_date)
    }
}
