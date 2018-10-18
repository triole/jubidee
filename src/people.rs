use chrono;
use chrono::offset::{Local, TimeZone};
use chrono::prelude::*;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Data {
    now: chrono::DateTime<chrono::Local>,
    people: Vec<Person>,
}

impl Data {
    pub fn init() -> Self {
        let d = Data {
            now: Local::now(),
            people: Vec::new(),
        };
        return d;
    }

    pub fn add(&mut self, name: &str, dob: &str) {
        let dob_date = self.parse_date_string(dob);
        let p = Person {
            name: name.to_string(),
            dob: dob_date,
            age: self.date_delta(&self.now, &dob_date),
            next_birthday: self.next_birthday(&dob_date),
        };
        self.people.push(p);
    }

    fn parse_date_string(&self, s: &str) -> chrono::DateTime<chrono::Local> {
        return Local.datetime_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap();
    }

    fn date_delta(
        &self,
        d1: &chrono::DateTime<chrono::Local>,
        d2: &chrono::DateTime<chrono::Local>,
    ) -> chrono::Duration {
        return d2.signed_duration_since(d1.to_owned());
    }

    fn next_birthday(
        &self,
        dob: &chrono::DateTime<chrono::Local>,
    ) -> chrono::DateTime<chrono::Local> {
        let current_year = self.now.year();
        let mut next_birthday = dob.with_year(current_year).unwrap();
        if next_birthday < self.now {
            next_birthday = dob.with_year(current_year + 1).unwrap();
        }
        return next_birthday;
    }
}

#[derive(Debug, Eq)]
pub struct Person {
    name: String,
    dob: chrono::DateTime<chrono::Local>,
    age: chrono::Duration,
    next_birthday: chrono::DateTime<chrono::Local>,
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
