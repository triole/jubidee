use chrono::offset::{Local, TimeZone};

use chrono;

struct Delta {
    duration: chrono::Duration
}

pub fn now() -> chrono::DateTime<chrono::Local>{
    return Local::now();
}

pub fn parse_string(s: &str)-> chrono::DateTime<chrono::Local>{
    return Local.datetime_from_str(&s, "%Y-%M-%d %H:%M:%S").unwrap();
}

pub fn date_delta(s1: &str, s2: &str) -> chrono::Duration{
    let d1 = parse_string(s1);
    let d2 = parse_string(s2);
    return d2.signed_duration_since(d1);
}
