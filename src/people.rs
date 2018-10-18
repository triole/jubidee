use chrono::offset::{Local, TimeZone};

use chrono;


pub fn data() -> Vec<Person>{
    let mut vec = Vec::new();
    vec.push(Person::new("Matea", "2017-12-30 17:40:00"));
    vec.push(Person::new("Flora", "2016-02-20 20:42:00"));
    vec.push(Person::new("Bär", "1981-11-22 12:00:00"));
    vec.push(Person::new("Er", "1981-08-07 11:55:00"));
    return vec;
}

#[derive(Debug)]
pub struct Person {
    name: String,
    dob: String,
    age: chrono::Duration,
}

impl Person {
    fn new(name: &str, dob: &str) -> Person {
        Person {
            name: name.to_string(),
            dob: dob.to_string(),
            age: date_delta(Local::now(), dob)
        }
    }
}

pub fn parse_string(s: &str)-> chrono::DateTime<chrono::Local>{
    return Local.datetime_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap();
}

pub fn date_delta(d1: chrono::DateTime<chrono::Local>, s2: &str) -> chrono::Duration{
    let d2 = parse_string(&s2);
    return d2.signed_duration_since(d1);
}
