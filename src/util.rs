use chrono;
use chrono::offset::{Local, TimeZone};
use chrono::prelude::*;

pub fn parse_date_string(s: &str) -> chrono::DateTime<chrono::Local> {
    if s == "now" {
        let now = Local::now()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        return now;
    } else {
        let t = s.to_owned() + " 00:00:00";
        return Local.datetime_from_str(&t, "%Y%m%d %H:%M:%S").unwrap();
    }
}
