use chrono;
use chrono::offset::{Local, TimeZone};

pub fn parse_date_string(s: &str) -> chrono::DateTime<chrono::Local> {
    let t = s.to_owned() + " 00:00:00";
    if s == "now" {
        return Local::now();
    } else {
        return Local.datetime_from_str(&t, "%Y%m%d %H:%M:%S").unwrap();
    }
}
