use chrono;
use chrono::prelude::*;
use chrono::offset::{Local, TimeZone};

use std::cmp::Ordering;

pub fn data() -> Vec<Person>{
    let mut vec = Vec::new();
    vec.push(Person::new("Matea", "2017-12-30 17:40:00"));
    vec.push(Person::new("Flora", "2016-02-20 20:42:00"));
    vec.push(Person::new("Bär", "1981-11-22 12:00:00"));
    vec.push(Person::new("Er", "1981-08-07 11:55:00"));
    vec.sort();
    return vec;
}

#[derive(Debug)]
#[derive(Eq)]
pub struct Person {
    name: String,
    dob: chrono::DateTime<chrono::Local>,
    age: chrono::Duration,
    next_birthday: chrono::DateTime<chrono::Local>,
}

impl Person {
    fn new(name: &str, dob: &str) -> Person {
        let dob_date = parse_date_string(dob);
        Person {
            name: name.to_string(),
            dob: dob_date,
            age: date_delta(&Local::now(), &dob_date),
            next_birthday: next_birthday(&dob_date)
        }
    }
}

// --- make the struct sortable by next birthday
impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.next_birthday.cmp(&other.next_birthday)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.next_birthday == other.next_birthday
    }
}
// --- end sortable

fn parse_date_string(s: &str)-> chrono::DateTime<chrono::Local>{
    return Local.datetime_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap();
}

fn date_delta(d1: &chrono::DateTime<chrono::Local>, d2: &chrono::DateTime<chrono::Local>) -> chrono::Duration{
    return d2.signed_duration_since(d1.to_owned());
}

fn next_birthday(dob: &chrono::DateTime<chrono::Local>) -> chrono::DateTime<chrono::Local>{
    let current_year = Local::now().year();
    let mut next_birthday = dob.with_year(current_year).unwrap();
    if next_birthday < Local::now() {
        next_birthday = dob.with_year(current_year+1).unwrap();
    }
    return next_birthday
}
