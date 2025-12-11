use chrono::{Datelike, NaiveDateTime, Timelike};
use serde::Serialize;

use crate::orm::query::TimeWithTickets;

#[derive(Debug, Serialize)]
pub struct CsvTime {
    pub time_start: String,
    pub time_end: String,
    pub time_dur: f64,
    pub tickets: String,
    pub time_desc: String,
}

impl From<TimeWithTickets> for CsvTime {
    fn from(value: TimeWithTickets) -> Self {
        CsvTime {
            time_start: write_date_time(value.time_start),
            time_end: write_date_time(value.time_end),
            time_dur: value.time_dur.unwrap(),
            time_desc: value.time_desc,
            tickets: value.tickets.into_iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}

pub fn write_date_time(dt: NaiveDateTime) -> String {
    format!(
        "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute()
    )
}