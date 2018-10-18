use chrono::offset::{Local, TimeZone};

use chrono;

struct Delta {
    duration: chrono::Duration
}

pub fn now() -> chrono::DateTime<chrono::Local>{
    return Local::now();
}

pub fn date_delta() -> chrono::Duration{
    let d1 = now();
    let d2 = Local.datetime_from_str(&"Jan 30 02:19:17 2018", "%b %d %H:%M:%S %Y").unwrap();
    return d2.signed_duration_since(d1);
    // return delta;
    // println!("Duration: {:?}", duration);
    // println!("As whole days: {:?}", duration.num_days());
}
