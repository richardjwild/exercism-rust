extern crate chrono;
use chrono::*;

const GIGASECOND_DURATION: i64 = 1000000000;

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    return start + Duration::seconds(GIGASECOND_DURATION);
}
