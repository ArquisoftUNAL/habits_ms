use chrono::NaiveDate;
use serde_derive::Serialize;

// Embedded models
#[derive(Debug, Serialize)]
pub struct Event {
    pub date: NaiveDate,
}

// Response models
#[derive(Debug, Serialize)]
pub struct EventsMultipleQueryResponse {
    pub message: String,
    pub events: Vec<Event>,
}
