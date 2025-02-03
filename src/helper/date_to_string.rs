use chrono::NaiveDateTime;

pub fn timestamp_to_string(timestamp: Option<NaiveDateTime>) -> Option<String> {
    match timestamp {
        Some(t) => Some(t.format("%Y-%m-%d %H:%M:%S").to_string()),
        None => None,
    }
}

use chrono::{DateTime, Utc};
use chrono_tz::Asia::Jakarta; // Import zona waktu Jakarta

pub fn timestamptz_to_string(timestamptz: Option<DateTime<Utc>>) -> Option<String> {
    // match timestamptz {
    //     Some(t) => Some(t.to_string()),
    //     None => None,
    // }
    timestamptz.map(|t| {
        t.with_timezone(&Jakarta)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    })
}
