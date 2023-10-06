use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde_derive::Serialize;

// Embedded models
#[derive(Debug, Serialize)]
pub struct Event {
    pub date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct CalendarEvent {
    pub date: NaiveDate,
    pub data: BigDecimal,
    pub relative_frequency: BigDecimal,
}

// Response models
#[derive(Debug, Serialize)]
pub struct EventsMultipleQueryResponse {
    pub message: String,
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize)]
pub struct CalendarEventsMultipleQueryResponse {
    pub message: String,
    pub events: Vec<CalendarEvent>,
}
