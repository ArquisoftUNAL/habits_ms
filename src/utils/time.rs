use chrono::{Datelike, Duration, Months, NaiveDate};

use crate::models::database::RecDataEnum;
use std::mem;

pub struct DateRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub recurrence_type: RecDataEnum,
    pub frequency_data: NaiveDate,

    // Function to get next date based on recurrence type
    pub get_next_date: fn(NaiveDate) -> NaiveDate,
}

impl DateRange {
    pub fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        recurrence_type: RecDataEnum,
        frequency_data: NaiveDate,
    ) -> Self {
        let mut start_date = start_date;
        let end_date = end_date;
        let recurrence_type = recurrence_type;
        let frequency_data = frequency_data;

        // Reference date represents the start of the habit, so we must match the start date with the reference date
        if start_date < frequency_data {
            start_date = frequency_data;
        }

        // When start date is less than end date, an offset must be applied to initial date

        // Get separation between dates
        let difference = frequency_data - start_date;

        let reference_day_of_week = frequency_data.weekday().num_days_from_monday();
        let start_day_of_week = start_date.weekday().num_days_from_monday();
        let reference_day_of_month = frequency_data.day();
        let start_day_of_month = start_date.day();

        let new_start_date = match recurrence_type {
            RecDataEnum::daily => start_date + Duration::days(difference.num_days()),
            RecDataEnum::weekly => {
                let offset_days = (reference_day_of_week - start_day_of_week) % 7;

                start_date + Duration::days(offset_days as i64)
            }
            RecDataEnum::weekly2 => {
                let full_weeks_difference_parity = difference.num_weeks() % 2;
                let offset_days = (reference_day_of_week - start_day_of_week) % 7;

                // Complete a week when difference is odd
                start_date
                    + Duration::days(offset_days as i64)
                    + Duration::weeks(full_weeks_difference_parity)
            }
            RecDataEnum::monthly => {
                if start_day_of_month < reference_day_of_month {
                    // Just a fixed days offset is needed
                    start_date
                        + Duration::days((reference_day_of_month - start_day_of_month) as i64)
                } else {
                    // Start day has passed reference day, so we must add a month and then subtract the difference
                    // to set the same day of the month
                    start_date.checked_add_months(Months::new(1)).unwrap()
                        + Duration::days(-((start_day_of_month - reference_day_of_month) as i64))
                }
            }
            RecDataEnum::monthly2 => {
                let reference_day_of_month = frequency_data.day();
                let reference_month = frequency_data.month();
                let start_day_of_month = start_date.day();
                let start_month = start_date.month();

                let month_difference_parity = (reference_month - start_month) % 2;

                if start_day_of_month < reference_day_of_month {
                    // Just a fixed days offset is needed
                    start_date
                        .checked_sub_months(Months::new(month_difference_parity))
                        .unwrap()
                        + Duration::days((reference_day_of_month - start_day_of_month) as i64)
                } else {
                    // Start day has passed reference day, so we must add a month and then subtract the difference
                    // to set the same day of the month
                    start_date
                        .checked_sub_months(Months::new(month_difference_parity))
                        .unwrap()
                        + Duration::days(-((start_day_of_month - reference_day_of_month) as i64))
                }
            }
        };

        // Now define a function for getting the next date based on recurrence type
        let get_next_date = match recurrence_type {
            RecDataEnum::daily => |date: NaiveDate| date + Duration::days(1),
            RecDataEnum::weekly => |date: NaiveDate| date + Duration::weeks(1),
            RecDataEnum::weekly2 => |date: NaiveDate| date + Duration::weeks(2),
            RecDataEnum::monthly => {
                |date: NaiveDate| date.checked_add_months(Months::new(1)).unwrap()
            }
            RecDataEnum::monthly2 => {
                |date: NaiveDate| date.checked_add_months(Months::new(2)).unwrap()
            }
        };

        start_date = new_start_date;
        DateRange {
            start_date,
            end_date,
            recurrence_type,
            frequency_data,

            get_next_date,
        }
    }
}

impl Iterator for DateRange {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        // Condition to end iteration
        if self.end_date < self.start_date {
            return None;
        }

        let next = (self.get_next_date)(self.start_date);
        Some(mem::replace(&mut self.start_date, next))
    }
}
